# Finance Tracker API

A robust RESTful API built with Rust and Actix-web for tracking personal finances. This API allows users to record transactions, manage their financial data, and get insights into their spending patterns.

## Features

- 🔐 **Authentication & Authorization**
  - JWT-based authentication
  - Role-based access control (User and Admin roles)
  - Secure password hashing with bcrypt

- 💰 **Transaction Management**
  - Record income and expenses
  - Categorize transactions
  - Track transaction dates
  - View transaction history
  - Get financial summaries (total income, expenses, and balance)

- 🛡️ **Security**
  - Password hashing
  - JWT token authentication
  - Role-based access control
  - Input validation
  - SQL injection prevention with SQLx

- 📊 **Database**
  - PostgreSQL database
  - SQLx for type-safe database queries
  - Automatic timestamp management
  - Proper indexing for performance

## Tech Stack

- **Backend Framework**: Actix-web
- **Database**: PostgreSQL
- **ORM**: SQLx
- **Authentication**: JWT
- **Password Hashing**: bcrypt
- **Validation**: validator
- **Serialization**: serde
- **Environment Variables**: dotenv
- **Logging**: env_logger

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- SQLx CLI (for database migrations)

## Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd finance_tracker
   ```

2. Create a `.env` file in the project root with the following variables:
   ```env
   DATABASE_URL=postgres://username:password@localhost:5432/finance_tracker
   JWT_SECRET=your-secret-key
   PORT=8080
   ```

3. Create the database:
   ```bash
   createdb finance_tracker
   ```

4. Run database migrations:
   ```bash
   sqlx migrate run
   ```

5. Build and run the project:
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:8080/api/v1`

## API Documentation

### Authentication

#### Register a new user
```http
POST /api/v1/users/register
Content-Type: application/json

{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "securepassword",
    "role": "USER"  // Optional, defaults to "USER"
}
```

#### Login
```http
POST /api/v1/users/login
Content-Type: application/json

{
    "username": "john_doe",
    "password": "securepassword"
}
```

### Transactions

#### Add a new transaction
```http
POST /api/v1/transactions
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
    "amount": 1000.50,
    "category": "Salary",
    "description": "Monthly salary",
    "date": "2024-03-15"
}
```

#### List user's transactions
```http
GET /api/v1/transactions
Authorization: Bearer <jwt_token>
```

#### Get financial summary
```http
GET /api/v1/transactions/summary
Authorization: Bearer <jwt_token>
```

### User Profile

#### Get user profile
```http
GET /api/v1/users/profile
Authorization: Bearer <jwt_token>
```

## Response Format

All API responses follow a consistent format:

```json
{
    "status": 200,
    "data": {
        // Response data here
    },
    "message": "Success message"
}
```

## Error Handling

The API uses standard HTTP status codes and returns error messages in a consistent format:

```json
{
    "status": 400,
    "data": null,
    "message": "Error message"
}
```

Common error codes:
- 400: Bad Request (validation errors)
- 401: Unauthorized (invalid or missing token)
- 403: Forbidden (insufficient permissions)
- 404: Not Found
- 500: Internal Server Error

## Development

### Running Tests
```bash
cargo test
```

### Database Migrations
```bash
# Create a new migration
sqlx migrate add <migration_name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Elie Mugenzi - [eliemugenzi@gmail.com](mailto:eliemugenzi@gmail.com) 