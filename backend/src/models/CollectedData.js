const mongoose = require('mongoose');
const Schema = mongoose.Schema;

/**
 * Schema for collected data from users
 */
const CollectedDataSchema = new Schema({
  // User reference
  user: {
    type: Schema.Types.ObjectId,
    ref: 'User',
    required: true
  },
  
  // Data type (location, contacts, calendar, sms, etc.)
  type: {
    type: String,
    required: true,
    enum: ['location', 'contacts', 'calendar', 'sms', 'other']
  },
  
  // Content of the data (depends on type)
  content: {
    type: Schema.Types.Mixed,
    required: true
  },
  
  // Metadata about the data collection
  metadata: {
    deviceInfo: {
      type: String,
      required: true
    },
    appVersion: {
      type: String,
      required: true
    },
    ipAddress: {
      type: String
    }
  },
  
  // Whether the data has been processed
  processed: {
    type: Boolean,
    default: false
  },
  
  // Rewards for data contribution
  rewards: {
    eligible: {
      type: Boolean,
      default: true
    },
    amount: {
      type: Number,
      default: 0
    },
    processed: {
      type: Boolean,
      default: false
    },
    transactionId: {
      type: String,
      default: null
    }
  },
  
  // Privacy level of the data
  privacyLevel: {
    type: String,
    enum: ['raw', 'anonymized', 'aggregated'],
    default: 'anonymized'
  }
}, {
  timestamps: true
});

// Create indexes
CollectedDataSchema.index({ user: 1, type: 1, createdAt: -1 });
CollectedDataSchema.index({ type: 1, createdAt: -1 });

// Add pre-save hook to ensure sensitive data is anonymized
CollectedDataSchema.pre('save', function(next) {
  // If data type is sensitive, ensure it's anonymized
  if (['contacts', 'sms'].includes(this.type) && this.privacyLevel === 'raw') {
    // Implement anonymization logic
    if (this.type === 'contacts') {
      this.content = this.anonymizeContacts(this.content);
    } else if (this.type === 'sms') {
      this.content = this.anonymizeSMS(this.content);
    }
    this.privacyLevel = 'anonymized';
  }
  next();
});

// Anonymization methods
CollectedDataSchema.methods = {
  // Implement different anonymization strategies based on data type
  anonymizeContacts(contacts) {
    // Example: Keep first letter of names, replace rest with *
    return contacts.map(contact => {
      if (contact.name) {
        const firstLetter = contact.name.charAt(0);
        const stars = '*'.repeat(contact.name.length - 1);
        contact.name = firstLetter + stars;
      }
      
      if (contact.phone) {
        // Only keep last 4 digits
        const length = contact.phone.length;
        contact.phone = `****${contact.phone.substring(length - 4)}`;
      }
      
      return contact;
    });
  },
  
  anonymizeSMS(sms) {
    // Obfuscate SMS content
    return sms.map(message => {
      if (message.content) {
        // Only store message length and keyword analysis result
        message.length = message.content.length;
        message.keywords = this.extractKeywords(message.content);
        message.content = null; // Don't store original content
      }
      return message;
    });
  },
  
  extractKeywords(text) {
    // Simple keyword extraction (just for example purposes)
    // In production, use a proper NLP library
    const commonKeywords = ['sale', 'offer', 'discount', 'payment', 'alert', 'reminder', 'meeting'];
    return commonKeywords.filter(keyword => text.toLowerCase().includes(keyword));
  }
};

const CollectedData = mongoose.model('CollectedData', CollectedDataSchema);

module.exports = CollectedData; 