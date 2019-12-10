export function get_file_content_from_js(key) {
  const value = window.map[key];
  if (value == null) {
    console.error(`${key} is not found. upload ${key}`);
    return [];
  }
  return window.map[key];
}
