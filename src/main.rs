extern crate async_std;
extern crate env_logger;
extern crate samotop;

use samotop::smtp::{Esmtp, EsmtpStartTls, Prudence, SmtpParser};
use samotop::mail::{Builder, DebugService, Name};



#[async_std::main]
async fn main() {
    use samotop::mail::*;
    use samotop::smtp::*;
    use samotop::server::*;
    env_logger::init();

    let mail = Builder
                + Esmtp.with(SmtpParser);

    let srv = TcpServer::on("0.0.0.0:25").serve(mail.build());

    srv.await.expect("success")
}
