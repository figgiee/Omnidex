import { ToastService } from '@/utils/toast';

// Frontend error types that match backend error structure
export interface StructuredError {
  type: 'Database' | 'Network' | 'Io' | 'InvalidInput' | 'NotFound' | 'Unknown';
  message: string;
}

export class ErrorHandler {
  /**
   * Parse error from Tauri command response and return user-friendly message
   */
  static parseError(error: any): { type: string; userMessage: string; fullError: any } {
    console.error('Full error details:', error);
    
    // Handle different error formats
    let errorType = 'Unknown';
    let userMessage = 'An unexpected error occurred';
    
    if (typeof error === 'string') {
      // Simple string errors
      if (error.includes('Database')) {
        errorType = 'Database';
        userMessage = 'Database operation failed. Please try again.';
      } else if (error.includes('Network') || error.includes('network') || error.includes('connection')) {
        errorType = 'Network';
        userMessage = 'Network error. Please check your internet connection.';
      } else if (error.includes('not found') || error.includes('Not found')) {
        errorType = 'NotFound';
        userMessage = 'The requested resource was not found.';
      } else if (error.includes('Invalid') || error.includes('invalid')) {
        errorType = 'InvalidInput';
        userMessage = 'Invalid input provided. Please check your data.';
      } else if (error.includes('permission') || error.includes('access')) {
        errorType = 'Permission';
        userMessage = 'Permission denied. Please check file access permissions.';
      } else {
        userMessage = error;
      }
    } else if (error && typeof error === 'object') {
      // Structured error object
      if (error.type) {
        errorType = error.type;
        switch (error.type) {
          case 'Database':
            userMessage = 'Database operation failed. Please try again.';
            break;
          case 'Network':
            userMessage = 'Could not connect to the marketplace. Please check your internet connection.';
            break;
          case 'Io':
            userMessage = 'File system error. Please check file permissions and disk space.';
            break;
          case 'InvalidInput':
            userMessage = 'Invalid input provided. Please check your data and try again.';
            break;
          case 'NotFound':
            userMessage = 'The requested resource was not found.';
            break;
          default:
            userMessage = error.message || 'An unexpected error occurred.';
        }
      } else if (error.message) {
        userMessage = error.message;
      }
    }
    
    return {
      type: errorType,
      userMessage,
      fullError: error
    };
  }

  /**
   * Show user-friendly toast notification for errors
   */
  static showErrorToast(error: any, title: string = 'Error') {
    const parsed = this.parseError(error);
    
    // Log for debugging
    console.error(`${title}: ${parsed.userMessage}`);
    
    // Show actual toast notification
    ToastService.showError(parsed.userMessage, title);
    
    return parsed;
  }

  /**
   * Show success toast notification
   */
  static showSuccessToast(message: string, title?: string) {
    ToastService.showSuccess(message, title);
  }

  /**
   * Show info toast notification
   */
  static showInfoToast(message: string, title?: string) {
    ToastService.showInfo(message, title);
  }
} 