use super::*;

mod arguments_test {
    // ccscan 192.10.0.1
    // ccscan 192.0.0.1 -j=100 | mentioning the no of parellel scan
    // ccscan 192.0.0.1 -p=8080,8000,.. | mentioning all the ports to scan
    // ccscan 192.0.0.* | for sweep scan
    // ccscan -h | for cli help

    use super::*;

    #[test]
    fn test_argument_count() {
        // It should fail for no arguments passed
        // It should fail for less than 2 arguments passed
        let result = Arguments::new(&[String::from("cli-app")]);
        assert_eq!(result, Err("Insufficient Arguments passed"));

        let res = Arguments::new(&[]);
        assert_eq!(res, Err("Insufficient Arguments passed"));
    }

    #[test]
    fn test_for_host_name() {
        let result = Arguments::new(&[String::from("cli-app"), String::from("192.0.0.1")]);
        assert_eq!(
            result,
            Ok(Arguments {
                host: IpAddr::V4("192.0.0.1".parse().unwrap()),
                flags: None,
                is_sweep: false
            })
        );
        let result = Arguments::new(&[String::from("cli-app"), String::from("1920.0.0.1")]);
        assert_eq!(result, Err("Invalid host name"));
    }

    #[test]
    fn test_for_j_flag() {
        let result = Arguments::new(&[
            String::from("cli-app"),
            String::from("192.0.0.1"),
            String::from("-j=100"),
        ]);
        assert_eq!(
            result,
            Ok(Arguments {
                host: IpAddr::V4("192.0.0.1".parse().unwrap()),
                flags: Some(vec![Flags {
                    name: String::from("j"),
                    value: vec![String::from("100")],
                }]),
                is_sweep: false,
            })
        );
    }

    #[test]
    fn test_for_port_flag() {
        let result = Arguments::new(&[
            String::from("cli-app"),
            String::from("192.0.0.1"),
            String::from("-p=8080,8081"),
        ]);
        assert_eq!(
            result,
            Ok(Arguments {
                host: IpAddr::V4("192.0.0.1".parse().unwrap()),
                flags: Some(vec![Flags {
                    name: String::from("p"),
                    value: vec![String::from("8080"), String::from("8081")],
                }]),
                is_sweep: false,
            })
        );
    }

    #[test]
    fn test_for_multiple_flags() {
        let result = Arguments::new(&[
            String::from("cli-app"),
            String::from("192.0.0.1"),
            String::from("-p=8080,8081"),
            String::from("-j=100"),
        ]);
        assert_eq!(
            result,
            Ok(Arguments {
                host: IpAddr::V4("192.0.0.1".parse().unwrap()),
                flags: Some(vec![
                    Flags {
                        name: String::from("p"),
                        value: vec![String::from("8080"), String::from("8081")],
                    },
                    Flags {
                        name: String::from("j"),
                        value: vec![String::from("100")],
                    }
                ]),
                is_sweep: false,
            })
        );
    }

    #[test]
    fn test_for_sweep_method() {
        let result = Arguments::new(&[String::from("cli-app"), String::from("192.0.0.*")]);
        assert_eq!(
            result.map(|mut args| {
                if let Some(ref mut flags) = args.flags {
                    flags.sort_by(|a, b| a.name.cmp(&b.name));
                }
                args
            }),
            Ok(Arguments {
                host: IpAddr::V4("192.0.0.1".parse().unwrap()),
                flags: None,
                is_sweep: true,
            })
        );
    }

    #[test]
    fn test_for_help_flag() {
        let result = Arguments::new(&[String::from("cli-app"), String::from("-h")]);
        assert_eq!(result, Err("Help flag passed"));
    }
}
