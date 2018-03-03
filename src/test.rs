use super::*;

#[test]
fn test_parse_file_name() {
    assert!(parse_file_name("".to_string()).is_none());
    assert!(parse_file_name("ac0v_6000000000_38109184_38109184".to_string()).is_none());
    assert!(parse_file_name("17274946210831421354_ac0v_38109184_38109184".to_string()).is_none());
    assert!(parse_file_name("17274946210831421354_6000000000_ac0v_38109184".to_string()).is_none());
    assert!(parse_file_name("17274946210831421354_6000000000_38109184_ac0v".to_string()).is_none());

    let plot_file = parse_file_name("17274946210831421354_6000000000_38109184_38109184".to_string()).unwrap();
    assert!(plot_file.optimized);
    assert_eq!(plot_file.account_id, 17274946210831421354);
    assert_eq!(plot_file.start_nonce, 6000000000);
    assert_eq!(plot_file.nonce_count, 38109184);
    assert_eq!(plot_file.stagger, 38109184);

    let plot_file = parse_file_name("17274946210831421354_6000000000_38109184_38109189".to_string()).unwrap();
    assert!(!plot_file.optimized);
}



