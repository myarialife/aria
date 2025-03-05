const winston = require('winston');
const path = require('path');
const fs = require('fs');

// Define log formats
const { format, transports } = winston;
const { combine, timestamp, printf, colorize, json } = format;

// Custom format for development environment
const myFormat = printf(({ level, message, timestamp }) => {
  return `${timestamp} ${level}: ${message}`;
});

// Development format
const developmentFormat = combine(
  colorize(),
  timestamp(),
  myFormat
);

// Create log directory
const logDir = path.join(__dirname, '../../logs');
if (!fs.existsSync(logDir)) fs.mkdirSync(logDir);

// Create logger
const logger = winston.createLogger({
  level: process.env.LOG_LEVEL || 'info',
  format: combine(timestamp(), json()),
  defaultMeta: { service: 'aria-backend' },
  transports: [
    // Write errors to error.log, everything else to combined.log
    new transports.File({
      filename: path.join(logDir, 'error.log'),
      level: 'error'
    }),
    new transports.File({
      filename: path.join(logDir, 'combined.log')
    })
  ]
});

// If not in production, also output to console
if (process.env.NODE_ENV !== 'production') {
  logger.add(new transports.Console({
    format: developmentFormat
  }));
}

module.exports = logger; 