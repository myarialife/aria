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

// 密码加密中间件
UserSchema.pre('save', async function(next) {
  const user = this;
  
  // 只有当密码被修改时才重新加密
  if (!user.isModified('password')) return next();
  
  try {
    // 生成盐
    const salt = await bcrypt.genSalt(10);
    // 使用盐哈希密码
    const hashedPassword = await bcrypt.hash(user.password, salt);
    // 替换原密码
    user.password = hashedPassword;
    next();
  } catch (error) {
    return next(error);
  }
});

// 密码验证方法
UserSchema.methods.comparePassword = async function(candidatePassword) {
  return bcrypt.compare(candidatePassword, this.password);
};

module.exports = mongoose.model('User', UserSchema);