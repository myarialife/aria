/**
 * ARIA Token Deployment Script
 * 
 * This script helps deploy the ARIA token contract to Solana devnet
 * and initializes the token with the correct allocations.
 */

const { 
  Connection, 
  PublicKey, 
  Keypair, 
  SystemProgram, 
  Transaction, 
  sendAndConfirmTransaction,
  SYSVAR_RENT_PUBKEY,
  SYSVAR_CLOCK_PUBKEY,
  TransactionInstruction
} = require('@solana/web3.js');
const { 
  Token, 
  TOKEN_PROGRAM_ID, 
  MintLayout,
  ASSOCIATED_TOKEN_PROGRAM_ID
} = require('@solana/spl-token');
const fs = require('fs');
const path = require('path');
const borsh = require('borsh');

// Network connection (devnet for testing)
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// Constants for ARIA token
const TOKEN_DECIMALS = 9;
const TOTAL_SUPPLY = 100_000_000_000_000_000n; // 100 million with 9 decimals
const USER_INCENTIVES = TOTAL_SUPPLY / 2n;      // 50%
const TEAM_DEVELOPMENT = TOTAL_SUPPLY / 5n;     // 20%
const COMMUNITY_GOVERNANCE = TOTAL_SUPPLY / 5n; // 20%
const MARKETING_PARTNERSHIPS = TOTAL_SUPPLY / 10n; // 10%

// Load keypair from file or create a new one
async function loadOrCreateKeypair(filePath) {
  try {
    const keypairData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
    return Keypair.fromSecretKey(new Uint8Array(keypairData));
  } catch (err) {
    console.log(`Creating new keypair at ${filePath}`);
    const keypair = Keypair.generate();
    fs.writeFileSync(filePath, JSON.stringify(Array.from(keypair.secretKey)));
    return keypair;
  }
}

// Create an account with the minimum rent
async function createAccount(connection, space, programId, owner, name) {
  console.log(`Creating ${name} account...`);
  const account = Keypair.generate();
  const rent = await connection.getMinimumBalanceForRentExemption(space);
  
  const transaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: owner.publicKey,
      newAccountPubkey: account.publicKey,
      lamports: rent,
      space: space,
      programId: programId,
    })
  );
  
  await sendAndConfirmTransaction(
    connection,
    transaction,
    [owner, account],
    { commitment: 'confirmed' }
  );
  
  console.log(`${name} account created: ${account.publicKey.toString()}`);
  return account;
}

// Create an associated token account
async function createAssociatedTokenAccount(connection, payer, ownerPubkey, mintPubkey) {
  const associatedTokenAddress = await Token.getAssociatedTokenAddress(
    ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID,
    mintPubkey,
    ownerPubkey
  );
  
  const transaction = new Transaction().add(
    Token.createAssociatedTokenAccountInstruction(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      mintPubkey,
      associatedTokenAddress,
      ownerPubkey,
      payer.publicKey
    )
  );
  
  await sendAndConfirmTransaction(
    connection,
    transaction,
    [payer],
    { commitment: 'confirmed' }
  );
  
  console.log(`Associated token account created: ${associatedTokenAddress.toString()}`);
  return associatedTokenAddress;
}

// Instructions serialization helpers
class AriaTokenInstructionSchema {
  constructor(properties) {
    Object.assign(this, properties);
  }
  
  static initializeMetadata(name, symbol, uri) {
    return new AriaTokenInstructionSchema({
      variant: 2, // InitializeMetadata instruction index
      name,
      symbol,
      uri
    });
  }
  
  static initializeAuthority() {
    return new AriaTokenInstructionSchema({
      variant: 7 // InitializeAuthority instruction index
    });
  }
  
  static addAuthority(newAuthority, role) {
    return new AriaTokenInstructionSchema({
      variant: 8, // AddAuthority instruction index
      newAuthority: newAuthority.toBuffer(),
      role
    });
  }
  
  static lockTokens(amount, lockDuration) {
    return new AriaTokenInstructionSchema({
      variant: 5, // LockTokens instruction index
      amount: BigInt(amount),
      lockDuration: BigInt(lockDuration)
    });
  }
}

// Borsh schema for serialization
const instructionSchema = new Map([
  [
    AriaTokenInstructionSchema,
    {
      kind: 'struct',
      fields: [
        ['variant', 'u8'],
        ['name', { kind: 'option', type: 'string' }],
        ['symbol', { kind: 'option', type: 'string' }],
        ['uri', { kind: 'option', type: 'string' }],
        ['amount', { kind: 'option', type: 'u64' }],
        ['lockDuration', { kind: 'option', type: 'u64' }],
        ['newAuthority', { kind: 'option', type: [32] }],
        ['role', { kind: 'option', type: 'u8' }]
      ]
    }
  ]
]);

// Create a transaction instruction using borsh serialization
function createAriaInstruction(programId, accounts, instruction) {
  const buffer = borsh.serialize(instructionSchema, instruction);
  return new TransactionInstruction({
    keys: accounts,
    programId,
    data: Buffer.from(buffer)
  });
}

async function main() {
  // Load deployer keypair
  const deployerKeypair = await loadOrCreateKeypair(
    path.resolve(__dirname, '../keys/deployer.json')
  );
  
  console.log('Deployer public key:', deployerKeypair.publicKey.toString());
  
  // Request airdrop if balance is low
  const balance = await connection.getBalance(deployerKeypair.publicKey);
  if (balance < 1 * 1000000000) { // 1 SOL in lamports
    console.log('Requesting airdrop...');
    const signature = await connection.requestAirdrop(
      deployerKeypair.publicKey,
      2 * 1000000000 // 2 SOL
    );
    await connection.confirmTransaction(signature);
    console.log('Airdrop successful');
  }
  
  // Create keypairs for token allocations
  const userIncentivesKeypair = await loadOrCreateKeypair(
    path.resolve(__dirname, '../keys/user_incentives.json')
  );
  const teamDevelopmentKeypair = await loadOrCreateKeypair(
    path.resolve(__dirname, '../keys/team_development.json')
  );
  const communityGovernanceKeypair = await loadOrCreateKeypair(
    path.resolve(__dirname, '../keys/community_governance.json')
  );
  const marketingPartnershipsKeypair = await loadOrCreateKeypair(
    path.resolve(__dirname, '../keys/marketing_partnerships.json')
  );
  
  console.log('User incentives public key:', userIncentivesKeypair.publicKey.toString());
  console.log('Team development public key:', teamDevelopmentKeypair.publicKey.toString());
  console.log('Community governance public key:', communityGovernanceKeypair.publicKey.toString());
  console.log('Marketing partnerships public key:', marketingPartnershipsKeypair.publicKey.toString());
  
  // Get the program ID (this should be your deployed program ID)
  // For now, we'll generate a dummy one for demonstration
  const programKeypair = await loadOrCreateKeypair(
    path.resolve(__dirname, '../keys/program.json')
  );
  const programId = programKeypair.publicKey;
  console.log('Program ID:', programId.toString());
  
  // Create a new token mint
  const tokenMint = Keypair.generate();
  console.log('Token mint public key:', tokenMint.publicKey.toString());
  
  // Calculate the space required for the mint
  const mintSpace = MintLayout.span;
  const mintRent = await connection.getMinimumBalanceForRentExemption(mintSpace);
  
  // Create a transaction to create the mint account
  const createMintTransaction = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: deployerKeypair.publicKey,
      newAccountPubkey: tokenMint.publicKey,
      lamports: mintRent,
      space: mintSpace,
      programId: TOKEN_PROGRAM_ID,
    })
  );
  
  try {
    await sendAndConfirmTransaction(
      connection,
      createMintTransaction,
      [deployerKeypair, tokenMint],
      { commitment: 'confirmed' }
    );
    console.log('Mint account created successfully');
  } catch (err) {
    console.error('Error creating mint account:', err);
    return;
  }
  
  // Initialize the token mint
  const token = new Token(
    connection,
    tokenMint.publicKey,
    TOKEN_PROGRAM_ID,
    deployerKeypair
  );
  
  try {
    await token.init(
      TOKEN_DECIMALS, // 9 decimals
      deployerKeypair.publicKey,
      deployerKeypair.publicKey
    );
    console.log('Token initialized successfully');
  } catch (err) {
    console.error('Error initializing token:', err);
    return;
  }
  
  // Create metadata account
  const metadataAccount = await createAccount(
    connection, 
    1000, // Enough space for metadata
    programId, 
    deployerKeypair,
    'Token Metadata'
  );
  
  // Initialize metadata
  try {
    const initializeMetadataInstruction = createAriaInstruction(
      programId,
      [
        { pubkey: metadataAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: tokenMint.publicKey, isSigner: false, isWritable: false },
        { pubkey: deployerKeypair.publicKey, isSigner: true, isWritable: false },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }
      ],
      AriaTokenInstructionSchema.initializeMetadata(
        "ARIA Token", 
        "ARI", 
        "https://ariatoken.io/metadata.json"
      )
    );
    
    const transaction = new Transaction().add(initializeMetadataInstruction);
    
    await sendAndConfirmTransaction(
      connection,
      transaction,
      [deployerKeypair, metadataAccount],
      { commitment: 'confirmed' }
    );
    
    console.log('Token metadata initialized successfully');
  } catch (err) {
    console.error('Error initializing token metadata:', err);
  }
  
  // Initialize authority account
  const authorityAccount = await createAccount(
    connection, 
    1000, // Enough space for authority list
    programId, 
    deployerKeypair,
    'Authority'
  );
  
  // Initialize authority
  try {
    const initializeAuthorityInstruction = createAriaInstruction(
      programId,
      [
        { pubkey: authorityAccount.publicKey, isSigner: false, isWritable: true },
        { pubkey: deployerKeypair.publicKey, isSigner: true, isWritable: false }
      ],
      AriaTokenInstructionSchema.initializeAuthority()
    );
    
    const transaction = new Transaction().add(initializeAuthorityInstruction);
    
    await sendAndConfirmTransaction(
      connection,
      transaction,
      [deployerKeypair, authorityAccount],
      { commitment: 'confirmed' }
    );
    
    console.log('Authority initialized successfully');
  } catch (err) {
    console.error('Error initializing authority:', err);
  }
  
  // Create token accounts for each allocation
  console.log('Creating token accounts...');
  
  try {
    const userIncentivesAccount = await token.createAccount(userIncentivesKeypair.publicKey);
    console.log('User incentives token account:', userIncentivesAccount.toString());
    
    const teamDevelopmentAccount = await token.createAccount(teamDevelopmentKeypair.publicKey);
    console.log('Team development token account:', teamDevelopmentAccount.toString());
    
    const communityGovernanceAccount = await token.createAccount(communityGovernanceKeypair.publicKey);
    console.log('Community governance token account:', communityGovernanceAccount.toString());
    
    const marketingPartnershipsAccount = await token.createAccount(marketingPartnershipsKeypair.publicKey);
    console.log('Marketing partnerships token account:', marketingPartnershipsAccount.toString());
    
    // Mint tokens to each allocation
    console.log('Minting tokens to allocations...');
    
    await token.mintTo(
      userIncentivesAccount,
      deployerKeypair.publicKey,
      [],
      Number(USER_INCENTIVES)
    );
    console.log(`Minted ${USER_INCENTIVES} tokens to user incentives allocation`);
    
    await token.mintTo(
      teamDevelopmentAccount,
      deployerKeypair.publicKey,
      [],
      Number(TEAM_DEVELOPMENT)
    );
    console.log(`Minted ${TEAM_DEVELOPMENT} tokens to team development allocation`);
    
    await token.mintTo(
      communityGovernanceAccount,
      deployerKeypair.publicKey,
      [],
      Number(COMMUNITY_GOVERNANCE)
    );
    console.log(`Minted ${COMMUNITY_GOVERNANCE} tokens to community governance allocation`);
    
    await token.mintTo(
      marketingPartnershipsAccount,
      deployerKeypair.publicKey,
      [],
      Number(MARKETING_PARTNERSHIPS)
    );
    console.log(`Minted ${MARKETING_PARTNERSHIPS} tokens to marketing partnerships allocation`);
    
    // Create lock accounts for team and marketing allocations
    const teamLockAccount = await createAccount(
      connection, 
      100, // Space for lock data
      programId, 
      deployerKeypair,
      'Team Lock'
    );
    
    const marketingLockAccount = await createAccount(
      connection, 
      100, // Space for lock data
      programId, 
      deployerKeypair,
      'Marketing Lock'
    );
    
    // Create vault accounts
    const teamVaultAccount = await token.createAccount(deployerKeypair.publicKey);
    console.log('Team vault token account:', teamVaultAccount.toString());
    
    const marketingVaultAccount = await token.createAccount(deployerKeypair.publicKey);
    console.log('Marketing vault token account:', marketingVaultAccount.toString());
    
    // Lock team tokens (1 year)
    const YEAR_IN_SECONDS = 31536000;
    try {
      // First transfer tokens to deployer
      await token.transfer(
        teamDevelopmentAccount,
        teamVaultAccount,
        teamDevelopmentKeypair.publicKey,
        [],
        Number(TEAM_DEVELOPMENT)
      );
      
      // Then lock them
      const lockTeamTokensInstruction = createAriaInstruction(
        programId,
        [
          { pubkey: teamLockAccount.publicKey, isSigner: false, isWritable: true },
          { pubkey: teamDevelopmentAccount, isSigner: false, isWritable: true },
          { pubkey: teamVaultAccount, isSigner: false, isWritable: true },
          { pubkey: deployerKeypair.publicKey, isSigner: true, isWritable: false },
          { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false }
        ],
        AriaTokenInstructionSchema.lockTokens(
          Number(TEAM_DEVELOPMENT),
          YEAR_IN_SECONDS
        )
      );
      
      const transaction = new Transaction().add(lockTeamTokensInstruction);
      
      await sendAndConfirmTransaction(
        connection,
        transaction,
        [deployerKeypair, teamLockAccount],
        { commitment: 'confirmed' }
      );
      
      console.log('Team tokens locked successfully for 1 year');
    } catch (err) {
      console.error('Error locking team tokens:', err);
    }
    
    // Lock marketing tokens (6 months)
    const SIX_MONTHS_IN_SECONDS = 15768000;
    try {
      // First transfer tokens to deployer
      await token.transfer(
        marketingPartnershipsAccount,
        marketingVaultAccount,
        marketingPartnershipsKeypair.publicKey,
        [],
        Number(MARKETING_PARTNERSHIPS)
      );
      
      // Then lock them
      const lockMarketingTokensInstruction = createAriaInstruction(
        programId,
        [
          { pubkey: marketingLockAccount.publicKey, isSigner: false, isWritable: true },
          { pubkey: marketingPartnershipsAccount, isSigner: false, isWritable: true },
          { pubkey: marketingVaultAccount, isSigner: false, isWritable: true },
          { pubkey: deployerKeypair.publicKey, isSigner: true, isWritable: false },
          { pubkey: SYSVAR_CLOCK_PUBKEY, isSigner: false, isWritable: false }
        ],
        AriaTokenInstructionSchema.lockTokens(
          Number(MARKETING_PARTNERSHIPS),
          SIX_MONTHS_IN_SECONDS
        )
      );
      
      const transaction = new Transaction().add(lockMarketingTokensInstruction);
      
      await sendAndConfirmTransaction(
        connection,
        transaction,
        [deployerKeypair, marketingLockAccount],
        { commitment: 'confirmed' }
      );
      
      console.log('Marketing tokens locked successfully for 6 months');
    } catch (err) {
      console.error('Error locking marketing tokens:', err);
    }
    
    console.log('ARIA token deployment complete!');
    console.log('Token Mint Address:', tokenMint.publicKey.toString());
    console.log('Metadata Account:', metadataAccount.publicKey.toString());
    console.log('Authority Account:', authorityAccount.publicKey.toString());
    
  } catch (err) {
    console.error('Error during token distribution:', err);
  }
}

main()
  .then(() => process.exit(0))
  .catch(err => {
    console.error(err);
    process.exit(1);
  }); 