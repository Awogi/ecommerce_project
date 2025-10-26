# School Uniform Store - Frontend

A modern, responsive web application built with Rust and Yew for managing school uniform orders.

## Features

- **Modern UI**: Clean, responsive design optimized for all devices
- **School Selection**: Browse uniforms by different schools and grades
- **Advanced Filtering**: Filter by school, grade, category, size, and price range
- **Shopping Cart**: Add items to cart, manage quantities, and proceed to checkout
- **User Authentication**: Login/registration system for students and parents
- **Order Management**: View order history and track order status
- **Real-time Updates**: Dynamic content loading and state management

## Technology Stack

- **Frontend Framework**: [Yew](https://yew.rs/) - Modern Rust framework for creating web apps with WebAssembly
- **Router**: [Yew Router](https://github.com/yewstack/yew/tree/master/packages/yew-router) - Client-side routing
- **HTTP Client**: [gloo-net](https://github.com/rustwasm/gloo) - HTTP requests and networking
- **Build Tool**: [Trunk](https://trunkrs.dev/) - Build and bundle Rust WASM projects
- **Styling**: Custom CSS with modern design principles

## Getting Started

### Prerequisites

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **WebAssembly target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Trunk** (build tool)
   ```bash
   cargo install trunk
   ```

### Development Setup

1. **Navigate to frontend directory**
   ```bash
   cd frontend
   ```

2. **Build dependencies**
   ```bash
   cargo build
   ```

3. **Start development server**
   ```bash
   trunk serve
   ```

4. **Open in browser**
   Navigate to `http://localhost:8080`

### Building for Production

1. **Build optimized version**
   ```bash
   trunk build --release
   ```

2. **Output**
   Built files will be in the `dist/` directory

## Project Structure

```
frontend/
├── src/
│   ├── components/          # Reusable UI components
│   │   ├── header.rs       # Navigation header with auth
│   │   ├── footer.rs       # Site footer with links
│   │   ├── product_card.rs # Product display component
│   │   ├── cart_summary.rs # Shopping cart summary
│   │   ├── filter_sidebar.rs # Product filtering sidebar
│   │   ├── loading.rs      # Loading spinners/indicators
│   │   └── modal.rs        # Modal dialog system
│   ├── pages/              # Main application pages
│   │   ├── home.rs         # Landing page with hero section
│   │   ├── catalog.rs      # Product catalog with filtering
│   │   ├── cart.rs         # Shopping cart management
│   │   ├── login.rs        # Authentication forms
│   │   └── profile.rs      # User profile and order history
│   ├── services/           # API and external services
│   │   └── api.rs          # Backend API client
│   ├── models.rs           # Data structures and types
│   ├── app.rs             # Main application component with routing
│   ├── main.rs            # Application entry point
│   └── styles.css         # Global styles and responsive design
├── Cargo.toml             # Rust dependencies and metadata
├── Trunk.toml             # Build configuration
└── index.html             # HTML template with meta tags
```

## API Integration

The frontend communicates with a Rust backend API that provides:

- User authentication and authorization
- Product catalog management
- Shopping cart operations
- Order processing and history
- School and grade data

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

## Development Commands

```bash
# Check code for errors
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Start development server
trunk serve

# Build for production
trunk build --release

# Clean build artifacts
cargo clean
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## License

This project is licensed under the MIT License.