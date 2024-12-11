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
                sweep_octact: None
            })
        );
        let result = Arguments::new(&[String::from("cli-app"), String::from("1920.0.0.1")]);
        assert_eq!(result, Err("Invalid host name"));

        let result = Arguments::new(&[String::from("cli-app"), String::from("localhost")]);
        assert_eq!(
            result,
            Ok(Arguments {
                host: IpAddr::V4("127.0.0.1".parse().unwrap()),
                flags: None,
                sweep_octact: None
            })
        );

        let result = Arguments::new(&[
            String::from("cli-app"),
            String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"),
        ]);
        assert!(result.is_ok());
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
                sweep_octact: None,
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
                sweep_octact: None,
            })
        );

        let result = Arguments::new(&[
            String::from("cli-app"),
            String::from("192.0.0.1"),
            String::from("-p=65536"),
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn test_for_invalid_flags() {
        let result = Arguments::new(&[
            String::from("cli-app"),
            String::from("192.0.0.1"),
            String::from("-f=8080,8081"),
        ]);
        assert_eq!(
            result,
            Ok(Arguments {
                host: IpAddr::V4("192.0.0.1".parse().unwrap()),
                flags: None,
                sweep_octact: None,
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
                sweep_octact: None,
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
                sweep_octact: Some(4),
            })
        );
    }

    #[test]
    fn test_for_help_flag() {
        let result = Arguments::new(&[String::from("cli-app"), String::from("-h")]);
        assert_eq!(result, Err("Help flag passed"));
    }
}
