const mongoose = require('mongoose');
const bcrypt = require('bcrypt');

const UserSchema = new mongoose.Schema({
  username: {
    type: String,
    required: true,
    unique: true,
    trim: true,
    minlength: 3,
    maxlength: 50
  },
  email: {
    type: String,
    required: true,
    unique: true,
    trim: true,
    lowercase: true
  },
  password: {
    type: String,
    required: true,
    minlength: 6
  },
  walletAddress: {
    type: String,
    unique: true,
    sparse: true
  },
  dataCollectionEnabled: {
    type: Boolean,
    default: false
  },
  dataCollectionPermissions: {
    location: { type: Boolean, default: false },
    contacts: { type: Boolean, default: false },
    calendar: { type: Boolean, default: false },
    sms: { type: Boolean, default: false }
  },
  role: {
    type: String,
    enum: ['user', 'admin'],
    default: 'user'
  },
  createdAt: {
    type: Date,
    default: Date.now
  },
  lastLogin: {
    type: Date
  },
  tokenBalance: {
    type: Number,
    default: 0
  }
}, {
  timestamps: true
});

// Password encryption middleware
UserSchema.pre('save', async function(next) {
  const user = this;
  
  // Only re-encrypt when password is modified
  if (!user.isModified('password')) {
    return next();
  }
  
  try {
    // Generate salt
    const salt = await bcrypt.genSalt(10);
    // Hash password with salt
    const hash = await bcrypt.hash(user.password, salt);
    // Replace original password
    user.password = hash;
    next();
  } catch (error) {
    next(error);
  }
});

// Password verification method
UserSchema.methods.comparePassword = async function(candidatePassword) {
  return bcrypt.compare(candidatePassword, this.password);
};

module.exports = mongoose.model('User', UserSchema);