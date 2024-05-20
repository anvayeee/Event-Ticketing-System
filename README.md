

# Event-Ticketing-System

The Event Ticketing System is a decentralized platform where event organizers can issue digital tickets as tokens on the Stellar network. These digital tickets can be easily transferred or sold, and entry can be validated through smart contracts, significantly reducing the risk of fraud. This project leverages the transparency, security, and efficiency of blockchain technology to revolutionize the event ticketing industry.


## Features

Decentralized Ticket Issuance: Event organizers can issue digital tickets as tokens on the Stellar network, ensuring transparency and security.

Easy Transfer and Sale: Tickets can be easily transferred or sold between users, providing flexibility and convenience.

Fraud Reduction: Smart contracts validate ticket authenticity and entry, reducing the risk of counterfeit tickets.

Fast and Low-Cost Transactions: Leveraging the Stellar network's capabilities for quick and economical transactions.

User-Friendly Interface: An intuitive interface for event organizers and attendees to manage and use tickets seamlessly.
## Installation

Install my-project with npm

```bash
  npm install my-project
  cd my-project
```
##  Setup Environment Variables

STELLAR_NETWORK=https://horizon-testnet.stellar.org
STELLAR_SECRET_KEY=your_stellar_secret_key
MONGODB_URI=your_mongodb_uri

## Run the Application

npm start


## Usage/Examples

## create an event

Log in to the platform.

Navigate to the "Create Event" section.

Fill in event details and issue digital tickets.

## Contract Functions
Sure, here are the contract function names with their one-liner descriptions:

1. `set_admin`: Sets the admin address.
2. `add_event_organizer`: Adds an event organizer with a specified ID and address.
3. `remove_event_organizer`: Removes an event organizer based on its ID.
4. `issue_ticket`: Issues a digital ticket as a token for a specified event.
5. `transfer_ticket`: Transfers a digital ticket from one address to another.
6. `validate_ticket`: Validates a digital ticket for entry to an event.
7. `get_event_details`: Retrieves the details of a specified event.
8. `get_ticket_details`: Retrieves the details of a specified ticket.
   
## Manage tickets

View issued tickets.

Transfer tickets to attendees.

Monitor ticket sales and transfers.

## Purchase tickets

Browse events on the platform.

Select an event and purchase tickets using your Stellar wallet.

## Transfer tickets

ransfer tickets to others easily through the platform.

## Validate Entry

Show your digital ticket at the event entry.

Smart contracts will validate the ticket for authenticity.

## Project Structure
.
├── contracts
│   └── EventTicketingSystem
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md

## Contributing

Contributions are always welcome!

Please adhere to this project's `code of conduct`.

We welcome contributions from the community. To contribute, please fork the repository and create a pull request with your changes. Ensure your code follows our coding standards and includes relevant tests.
## License

[MIT](https://choosealicense.com/licenses/mit/)

This project is licensed under the MIT License. See the LICENSE file for details.
## contact

For any questions or feedback, please open an issue on GitHub or contact us at support@example.com.
