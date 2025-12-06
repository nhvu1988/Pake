// Hook for Tauri to get current URL before closing window
window.pakeGetCurrentUrl = function() {
  try {
    return window.location?.href || null;
  } catch (e) {
    return null;
  }
};
