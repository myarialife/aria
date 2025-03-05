#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{
        program_pack::Pack,
        pubkey::Pubkey,
    };
    use spl_token::state::Mint;
    
    #[test]
    fn test_token_initialization() {
        // Create test addresses
        let program_id = Pubkey::new_unique();
        let mint_key = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        
        // Test initialization instruction creation
        let init_instruction = crate::instruction::initialize_token(
            &program_id,
            &mint_key,
            &mint_authority,
        );
        
        assert_eq!(init_instruction.program_id, program_id);
        assert_eq!(init_instruction.accounts.len(), 3);
    }
    
    #[test]
    fn test_token_distribution() {
        // Create test addresses
        let program_id = Pubkey::new_unique();
        let mint_key = Pubkey::new_unique();
        let user_incentives = Pubkey::new_unique();
        let team_development = Pubkey::new_unique();
        let community_governance = Pubkey::new_unique();
        let marketing_partnerships = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        
        // Test distribution instruction creation
        let distribute_instruction = crate::instruction::distribute_tokens(
            &program_id,
            &mint_key,
            &user_incentives,
            &team_development,
            &community_governance,
            &marketing_partnerships,
            &mint_authority,
        );
        
        assert_eq!(distribute_instruction.program_id, program_id);
        assert_eq!(distribute_instruction.accounts.len(), 6);
    }
} 