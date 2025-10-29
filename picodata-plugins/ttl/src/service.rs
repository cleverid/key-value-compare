use crate::config;
use crate::handlers::endpoints::http::routes;
use crate::handlers::endpoints::rpc::register_example_rpc_handle;
use shors::transport::http::server::Server;
use std::sync::LazyLock;
use std::time::{Duration};

use picodata_plugin::plugin::prelude::*;
use picodata_plugin::system::tarantool::log as t_log;
use picodata_plugin::system::tarantool::{say_info, say_error};
use picodata_plugin::background::CancellationToken;
use chrono::{Utc};

const TTL_JOB_NAME: &str = "ttl-worker";
static LOGGER: LazyLock<t_log::TarantoolLogger> = LazyLock::new(t_log::TarantoolLogger::default);

fn init_logger() {
    log::set_logger(&*LOGGER).map_or_else(
        |e| println!("failed to setup logger: {e:?}"),
        |()| log::set_max_level(log::LevelFilter::Trace),
    );
}

thread_local! {
    static HTTP_SERVER: LazyLock<Server> = LazyLock::new(Server::new);
}

#[derive(Debug, Default)]
pub struct ExampleService {}

impl Service for ExampleService {
    type Config = config::ExampleService;

    fn on_config_change(
        &mut self,
        ctx: &PicoContext,
        _config: Self::Config,
        _config_old: Self::Config,
    ) -> CallbackResult<()> {
        ctx.cancel_tagged_jobs(TTL_JOB_NAME, Duration::from_secs(1))
            .map_err(|err| format!("failed to cancel tagged jobs on config change: {err}"))?;
        ctx.register_tagged_job(get_ttl_job(), TTL_JOB_NAME)
           .map_err(|err| format!("failed to register tagged jobs on config change: {err}"))?;
        Ok(())
    }

    fn on_start(&mut self, context: &PicoContext, _config: Self::Config) -> CallbackResult<()> {
        init_logger();

        log::warn!("Registering HTTP handle /hello");
        HTTP_SERVER.with(|srv| {
            routes()
                .into_iter()
                .for_each(|route| srv.register(Box::new(route)));
        });

        log::warn!("Registering RPC handle /greetings_rpc");
        register_example_rpc_handle(context);

        log::warn!("Registering job");
        let ttl_job = get_ttl_job();
        context.register_tagged_job(ttl_job, TTL_JOB_NAME)
            .map_err(|err| format!("failed to register tagged job: {err}"))?;

        Ok(())
    }

}

const TTL_QUERY: &str = r#"
    DELETE FROM profiles WHERE id in (
	    select id from profiles where expire <= $1::datetime LIMIT 100 
    )
"#;


fn get_ttl_job() -> impl Fn(CancellationToken) {
    move |ct: CancellationToken| {
            while ct.wait_timeout(Duration::from_secs(1)).is_err() {
                log::warn!("TTL loop");
                let expired: String = Utc::now().to_rfc3339();
                // log::warn!("==== {}", expired);
                match picodata_plugin::sql::query(&TTL_QUERY)
                .bind(expired)
                .execute() {
                    Ok(rows_affected) => {
                        say_info!("Cleaned {rows_affected:?} expired records");
                    },
                    Err(error) => {
                        say_error!("Error while cleaning expired records: {error:?}")
                    }
                };
            }
        say_info!("TTL worker stopped");
    }
}

