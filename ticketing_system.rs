#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, vec};

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    Tickets,
}

#[derive(Clone)]
#[contracttype]
pub struct Ticket {
    id: u64,
    requester: Address,
    subject: String,
    description: String,
    status: TicketStatus,
}

#[derive(Clone)]
#[contracttype]
pub enum TicketStatus {
    Open,
    InProgress,
    Closed,
}

#[contract]
pub struct TicketingSystemContract;

#[contractimpl]
impl TicketingSystemContract {
    pub fn create_ticket(
        env: Env,
        requester: Address,
        subject: String,
        description: String,
    ) -> u64 {
        // Generate a unique ID for the ticket
        let ticket_id = env.storage().instance().get::<_, u64>(&StorageKey::Tickets).unwrap_or(0) + 1;

        // Create a new ticket with the provided details and set its status to "Open"
        let ticket = Ticket {
            id: ticket_id,
            requester,
            subject,
            description,
            status: TicketStatus::Open,
        };

        // Retrieve the existing tickets or create a new empty vector
        let mut tickets: Vec<Ticket> = env.storage().instance().get(&StorageKey::Tickets).unwrap();

        // Add the new ticket to the list of existing tickets
        tickets.push_back(ticket);

        // Update the tickets in storage
        env.storage().instance().set(&StorageKey::Tickets, &tickets);

        // Return the ID of the created ticket
        ticket_id
    }

    pub fn get_ticket(env: Env, ticket_id: u64) -> Option<Ticket> {
        // Retrieve the existing tickets
        let tickets: Vec<Ticket> = env.storage().instance().get(&StorageKey::Tickets).unwrap();

        // Find and return the ticket with the specified ID
        tickets.iter().find(|ticket| ticket.id == ticket_id).clone()
    }

 pub fn update_ticket_status(env: Env, ticket_id: u64, new_status: TicketStatus) -> bool {
    // Retrieve the existing tickets
    let mut tickets: Vec<Ticket> = env.storage().instance().get(&StorageKey::Tickets).unwrap();

    // Flag to indicate if the ticket was found and updated
    let mut ticket_updated = false;

    // Iterate over the vector using indices
    for i in 0..tickets.len() {
        // Check if the current ticket's ID matches the specified ID
        if tickets[i].id == ticket_id {
            // Update the status of the ticket
            tickets[i].status = new_status.clone();
            // Set the flag to indicate that the ticket was found and updated
            ticket_updated = true;
            // Exit the loop early since we found the ticket
            break;
        }
    }

    // If the ticket was found and updated, save the changes to storage
    if ticket_updated {
        env.storage().instance().set(&StorageKey::Tickets, &tickets);
    }

    ticket_updated // Return true if the ticket was found and updated, otherwise false
}



}
