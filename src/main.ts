import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

const historyDiv = document.getElementById('history');
const MAX_ITEMS = 50;

// Listen for the event emitted by Rust
listen<string>('clipboard-update', (event) => {
  const text = event.payload;

  const item = document.createElement('div');
  item.className = 'item';
  item.textContent = text;

  // Click to copy the item to clipboard
  item.onclick = async () => {
    await invoke('set_clipboard', { text });
  };

  // Add the newest items to the top of the history list
  historyDiv?.prepend(item);

  while (historyDiv && historyDiv.children.length > MAX_ITEMS) {
    historyDiv.removeChild(historyDiv.lastChild!);
  }
});