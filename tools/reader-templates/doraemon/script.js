(function () {
  'use strict';
  var reader = window.readerTemplate;
  function period() {
    var time = /(?:^|\s)(\d{1,2}):\d{2}/.exec(String(reader.fields.time || ''));
    var hour = time ? Number(time[1]) : new Date().getHours();
    return hour >= 19 || hour < 6 ? 'night' : hour >= 16 ? 'evening' : 'day';
  }
  function decorate(page) {
    var next = period();
    if (page.getAttribute('data-dora-period') !== next) page.setAttribute('data-dora-period', next);
  }
  reader.on('beforePage', function (event) { decorate(event.page); });
  reader.on('fieldsChange', function () { reader.pages.forEach(decorate); }, {fields: ['time']});
  // No timers, scroll listeners, per-frame DOM writes or network requests.
  // The header, footer and artwork stay still while the central reading frame scrolls.
})();
