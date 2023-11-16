//#![cfg_attr(not(feature = "std"), no_std)]

//! # A Concordium V1 smart contract
use concordium_std::{*,collections::BTreeMap };
use core::fmt::Debug;

type VotingOption = String

/// Your smart contract state.
#[derive(Serialize, SchemaType)]
pub struct State {
    pub description: String,
    pub options: Vec<VotingOption>,
    pub ballots: BTreeMap<AccountAddress, VotingIndex>,
    pub end_time: Timestamp,
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// enum User {
//     Anonymous {
//         id: u32,
//     },
//     Registered {
//         id: u32,
//         profile_picture: String,
//     }
// }

#[derive(Serialize, SchemaType)]

pub struct InitParameter {
    pub description: String,
    pub options: Vec<VotingOption>,
    pub end_time: Timestamp,
}


/// Init function that creates a new smart contract.
#[init(contract = "vote", parameter = "InitParameter")] //@audit contract Name
fn init(_ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<State> {
    // Your code
    // read given parameter
    let parameter: InitParameter = _ctx.parameter_cursor().get()?;

    // create initial state
    let state: State = State{
        description: parameter.description,
        options: parameter.options,
        end_time: parameter.end_time,
        ballots: BTreeMap::new(),
    }

    Ok(State)
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
pub enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// Your error
    VotingFinished,
}


//@audit define overall high level Contract

#[receive(
    contract = "vote",
    name = "Vote",
    parameter = "VotingOption",
    error = "Error",
    mutable
)]
fn vote(ctx: &ReceiveContext, _host: &mut Host<State>) -> Result<(), Error> {
    // Your code

    // let throw_error = ctx.parameter_cursor().get()?; // Returns Error::ParseError on failure
    // if throw_error {
    //     Err(Error::YourError)
    // } else {
    //     Ok(())
    // }

    // the election hasn't expired
    if ctx.metadata().slot_time() > host.state().end_time {
        return Err(Error::VotingFinished);
    }
    //that only accounts can vote
    //read voting option parameter
    //add or update the vote for the account
    //return ok if everything is ok

    Ok(());
}

/// View function that returns the content of the state.
#[receive(contract = "vote", name = "view", return_value = "State")]
fn view<'b>(_ctx: &ReceiveContext, host: &'b Host<State>) -> ReceiveResult<&'b State> {
    Ok(host.state())
}
