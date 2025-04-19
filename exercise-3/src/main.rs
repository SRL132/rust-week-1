fn main() {
    println!("Hello, world!");
}


pub fn deposit(ctx: Context<Deposit>, collat: u64) -> Result<()> {
    let rate = exchange_rate.deposit_rate as u128;
    let amt = (collat as u128 * rate / DECIMALS_SCALAR) as u64; 

            // transfer(token, from, to, amount)
    token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;

            // mint_to(token, to, amount)
    token::mint_to(shares_token, ctx.caller, amt)?;

    Ok(())
}
//exchange_rate.deposit_rate is an u64
//DECIMALS_SCALAR value is not given. It is an u128 which may have any value except 0.

//Explain what the function does
//Classic Deposit function where the user deposits collateral tokens and receives shares tokens in return according to the deposit rate

//What could go wrong?
//amt is downcasted to u64, if the rate is too high it could overflow, causing the user to get less shares than expected

//How to fix it?
//Use u128 for amt and rate, and use a constant for DECIMALS_SCALAR
//assuming mint_to expects to get u64, add overflow check before downcasting

pub fn depositSolution(ctx: Context<Deposit>, collat: u64) -> Result<()> {
    let rate = exchange_rate.deposit_rate; //u64
    let rate = exchange_rate.deposit_rate as u128;
    let amt = collat as u128 * rate / DECIMALS_SCALAR; 
    
    //add overflow check
    if amt > u64::MAX {
        return Err(Error::AMOUNT_TOO_HIGH);
    }

            // transfer(token, from, to, amount)
    token::transfer(collateral_token, ctx.caller, ctx.this, collat)?;

            // mint_to(token, to, amount)
    token::mint_to(shares_token, ctx.caller, amt as u64)?;

    Ok(())
}
