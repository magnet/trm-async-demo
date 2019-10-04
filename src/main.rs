#[derive(Debug, PartialEq, Eq)]
struct User {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
}

async fn get_user(user_id: u32) -> Result<User, Box<dyn std::error::Error>> {
    tokio::timer::delay_for(std::time::Duration::from_secs(1)).await;
    Ok(User {
        id: user_id,
        last_name: "Rust".into(),
        first_name: "Toulouse".into(),
    })
}

async fn get_user_by_first_name(user_first_name: &str) -> Result<User, Box<dyn std::error::Error>> {
    tokio::timer::delay_for(std::time::Duration::from_secs(2)).await;
    Ok(User {
        id: 42,
        last_name: user_first_name.into(),
        first_name: "Toulouse".into(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting user...");

    let user = get_user(42).await?;

    println!("User: {:?}", user);

    println!("Getting user by name...");

    let user_first_name = &user.first_name;

    let user = get_user_by_first_name(user_first_name).await?;

    println!("User: {:?}", user);

    Ok(())
}
