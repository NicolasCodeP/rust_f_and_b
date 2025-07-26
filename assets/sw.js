var cacheName = 'egui-template-pwa';
var filesToCache = [
  './',
  './index.html',
  './eframe_todo_list.js',
  './eframe_todo_list_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline, but skip if #dev is in the URL */
self.addEventListener('fetch', function (e) {
  // Check if the request URL contains '#dev'
  if (e.request.url.includes('#dev')) {
    // Always fetch from network, skip cache
    e.respondWith(fetch(e.request));
  } else {
    e.respondWith(
      caches.match(e.request).then(function (response) {
        return response || fetch(e.request);
      })
    );
  }
});
