var cacheName = 'egui-fb-management-v2';
var filesToCache = [
  './',
  './index.html',
  './eframe_todo_list.js',
  './eframe_todo_list_bg.wasm',
  './assets/manifest.json',
  './assets/icon-256.png',
  './assets/icon_ios_touch_192.png',
  './assets/maskable_icon_x512.png'
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    }).then(function() {
      return self.skipWaiting();
    })
  );
});

self.addEventListener('activate', function(e) {
  e.waitUntil(
    caches.keys().then(function(keyList) {
      return Promise.all(keyList.map(function(key) {
        if(key !== cacheName) {
          return caches.delete(key);
        }
      }));
    })
  );
  return self.clients.claim();
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  // Skip non-GET requests
  if (e.request.method !== 'GET') {
    return;
  }
  
  e.respondWith(
    caches.match(e.request).then(function (response) {
      // Return cached version or fetch from network
      return response || fetch(e.request).catch(function() {
        // If both cache and network fail, return a basic offline page for HTML requests
        if (e.request.headers.get('accept').includes('text/html')) {
          return new Response(
            '<html><body><h1>App is offline</h1><p>Please check your internet connection.</p></body></html>',
            { headers: { 'Content-Type': 'text/html' } }
          );
        }
      });
    })
  );
});
