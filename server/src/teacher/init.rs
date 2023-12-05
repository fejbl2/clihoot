use actix::{
    prelude::{Actor, Context},
    Addr, AsyncContext,
};
use actix_rt::System;

use crate::server::{lobby::Lobby, messages::RegisterTeacherMessage};

pub struct Teacher {
    pub lobby: Addr<Lobby>,
}

impl Actor for Teacher {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Teacher started, sending RegisterTeacherMessage to lobby");

        self.lobby.do_send(RegisterTeacherMessage {
            teacher: ctx.address(),
        });
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::prelude::Running {
        println!("Teacher stopping");
        actix::prelude::Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Teacher stopped");
    }
}

fn create_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
}

pub fn run_teacher(tx: Addr<Lobby>) {
    let system = actix::System::with_tokio_rt(create_tokio_runtime);

    system.block_on(init(tx));

    system.run().unwrap();
}

#[allow(clippy::unused_async)]
async fn init(lobby: Addr<Lobby>) {
    // spawn an actor for managing the lobby
    let _teacher_actor = Teacher { lobby }.start();

    // handle CTRL+C gracefully
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("CTRL-C received, shutting down");
        System::current().stop();
    });
}
