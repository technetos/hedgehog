# Hedgehog
HedgeHog is bearer authentication provider for Tide web applications.  

## Usage
```
async fn protected_echo_path(cx: Context<()>) -> EndpointResult<String> {
    cx.hedgehog().protect()?;

    let path: String = cx.param("path")?;
    
    Ok(format!("Your path is: {}", path))
}

fn main() {
    let mut app = tide::App::new(());
    app.at("/echo_path/:path").get(echo_path);
    app.serve("127.0.0.1:8000").unwrap();
}

```

## Hedgehog API
In Tide you can implement traits for the Context type to add functionality to
it.  The hedgehog API is made available through the `.hedgehog()` method on the
Context type. 
