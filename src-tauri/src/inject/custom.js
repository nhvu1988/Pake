// Hook for Tauri to get current URL before closing window
window.pakeGetCurrentUrl = function() {
  console.log('[Pake] pakeGetCurrentUrl called');
  try {
    const url = window.location?.href || null;
    console.log('[Pake] Current URL:', url);
    return url;
  } catch (e) {
    console.error('[Pake] Error getting URL:', e);
    return null;
  }
};

console.log('[Pake] custom.js loaded, pakeGetCurrentUrl registered');
