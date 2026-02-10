use crate::lambda_func::func;
use crate::schema::CustomEvent;
use dotenv;
use lambda_runtime::{Context, LambdaEvent};

#[tokio::test]
async fn test_lambda_func() {
    dotenv::dotenv().ok();

    let event = LambdaEvent::new(CustomEvent::mock(), Context::default());
    let result = func(event).await;

    match result {
        Ok(msg) => assert_eq!(msg, "success".to_string()),
        Err(e) => {
            println!("{:?}", e);
            panic!("Unexpected error")
        }
    }
}
