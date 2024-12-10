use super::*;

mod test_sniffer {
    use crate::args::Arguments;

    use super::*;
    use std::process::{Child, Command};
    use std::thread;
    use std::time::Duration;

    struct TestServer {
        server: Child,
    }

    impl TestServer {
        fn new(ports: &[i32]) -> Self {
            let port_args: Vec<String> = ports.iter().map(|port| port.to_string()).collect();
            let server = Command::new("go")
                .args(["run", "test_server/main.go"])
                .args(&port_args)
                .spawn()
                .expect("Failed to start test server");

            // Give the server a moment to start
            thread::sleep(Duration::from_secs(2));

            TestServer { server }
        }

        fn kill(&mut self) {
            println!("Server is killing");
            match self.server.kill() {
                Ok(_) => match self.server.wait() {
                    Ok(_) => println!("Server terminated successfully"),
                    Err(e) => println!("Error waiting for server: {}", e),
                },
                Err(e) => println!("Failed to kill server: {}", e),
            }
        }
    }

    #[test]
    fn test_valid_ip_address() {
        let ports = vec![80, 443];
        let ip = "localhost";
        let mut server = TestServer::new(&ports);

        let ip_args = Arguments::new(&[
            String::from("cli-app"),
            String::from(ip),
            String::from("-p=80,443"),
        ])
        .unwrap();
        let result = Sniffer::scan(ip_args);

        server.kill();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().open_ports, ports);
    }

    #[test]
    fn test_common_ports() {
        let ip = "localhost";
        let ports = vec![22, 80, 443, 3306, 5432];

        let server = TestServer::new(&ports);

        let ip_args = Arguments::new(&[String::from("cli-app"), String::from(ip)]).unwrap();
        let result = Sniffer::scan(ip_args);

        drop(server);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().open_ports, ports);
    }

    #[test]
    fn test_single_port() {
        let ip = "127.0.0.1";
        let ports = vec![8080];

        let ip_args = Arguments::new(&[String::from("cli-app"), String::from(ip)]).unwrap();
        let result = Sniffer::scan(ip_args);

        assert_eq!(result.unwrap().open_ports, ports)
    }
}
