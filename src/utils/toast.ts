// Simple fallback approach - just use console logging for now
// The toast will need to be called from within Vue components
function showToast(message: string, type: string, options: any = {}) {
  // Try to dispatch a custom event that components can listen to
  try {
    window.dispatchEvent(new CustomEvent('app-toast', {
      detail: { message, type, options }
    }));
  } catch (e) {
    // Fallback if custom events don't work
  }
}

export class ToastService {
  static showSuccess(message: string, title?: string) {
    const displayMessage = title ? `${title}: ${message}` : message;
    showToast(displayMessage, 'success', { timeout: 4000 });
  }

  static showError(message: string, title?: string) {
    const displayMessage = title ? `${title}: ${message}` : message;
    showToast(displayMessage, 'error', { timeout: 5000 });
  }

  static showInfo(message: string, title?: string) {
    const displayMessage = title ? `${title}: ${message}` : message;
    showToast(displayMessage, 'info', { timeout: 3000 });
  }

  static showWarning(message: string, title?: string) {
    const displayMessage = title ? `${title}: ${message}` : message;
    showToast(displayMessage, 'warning', { timeout: 4000 });
  }
} 