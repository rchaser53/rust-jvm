export function get_file_content_from_js(key) {
  const value = window.map[key];
  if (value == null) {
    console.error(`${key} is not found. upload ${key}`);
    return [];
  }
  return window.map[key];
}

export function output_log(value) {
  console.log(value);
  window.output.push(value);
}
