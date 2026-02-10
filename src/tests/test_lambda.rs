use crate::lambda_func::func;
use crate::schema::CustomEvent;
use lambda_runtime::{Context, LambdaEvent};

#[tokio::test]
async fn test_lambda_func() {
    let event = LambdaEvent::new(CustomEvent::mock(), Context::default());
    let result = func(event).await;

    match result {
        Ok(msg) => assert_eq!(msg, "success".to_string()),
        _ => panic!("expected Ok(msg), not Err"),
    }
}
