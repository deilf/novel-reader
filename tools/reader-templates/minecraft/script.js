(function () {
  'use strict';
  function chapterNumber(text) {
    var match = text.match(/第\s*([零〇一二三四五六七八九十百千万两\d]+)\s*[章回节卷]/);
    if (!match) return 'NEW';
    if (/^\d+$/.test(match[1])) return match[1];
    var digits = '零一二三四五六七八九', units = {十: 10, 百: 100, 千: 1000};
    var total = 0, section = 0, digit = 0;
    Array.prototype.forEach.call(match[1].replace(/〇/g, '零').replace(/两/g, '二'), function (character) {
      var value = digits.indexOf(character);
      if (value >= 0) digit = value;
      else if (character === '万') { total += (section + digit || 1) * 10000; section = 0; digit = 0; }
      else { section += (digit || 1) * units[character]; digit = 0; }
    });
    return String(total + section + digit);
  }
  // Classify the original semantic blocks before pagination. Decorations never
  // wrap, replace or duplicate source characters, images or paragraph actions.
  readerTemplate.on('beforeLayout', function () {
    var ordinal = 0;
    Array.prototype.forEach.call(readerTemplate.source.querySelectorAll('.reader-paragraph'), function (paragraph) {
      var text = paragraph.textContent.trim();
      var scene = /^[*＊※·•—_\-\s]{3,}$/.test(text);
      paragraph.classList.toggle('mc-scene', scene);
      paragraph.classList.toggle('mc-dialogue', /^[“「『"]/.test(text));
      paragraph.classList.toggle('mc-lead', ordinal === 0 && !!text && !scene);
      if (text && !scene) {
        ordinal++;
        paragraph.setAttribute('data-mc-ordinal', ordinal < 10 ? '0' + ordinal : String(ordinal));
      } else {
        paragraph.removeAttribute('data-mc-ordinal');
      }
    });
    Array.prototype.forEach.call(readerTemplate.source.querySelectorAll('.reader-chapter-title'), function (title) {
      title.setAttribute('data-mc-chapter', chapterNumber(title.textContent));
    });
  });
  var clockMinute = -1, animatedPage = null, animations = [], shotAnimations = [];
  var skyStops = [
    [0, '#0a1732', '#314a67'], [300, '#252544', '#be796f'],
    [390, '#517d9a', '#f3d59e'], [510, '#6baacb', '#c4e9dc'],
    [900, '#528eae', '#aedacf'], [1050, '#56577e', '#efb17b'],
    [1140, '#172d50', '#697397'], [1260, '#0a1732', '#314a67'], [1440, '#0a1732', '#314a67']
  ];
  function colorBetween(a, b, amount) {
    return '#' + [1, 3, 5].map(function (start) {
      var x = parseInt(a.slice(start, start + 2), 16), y = parseInt(b.slice(start, start + 2), 16);
      return ('0' + Math.round(x + (y - x) * amount).toString(16)).slice(-2);
    }).join('');
  }
  function updateClock() {
    // Native minute fields are also supplied to offscreen renderers. The same
    // page/fields therefore produce identical settled pixels in every WebView.
    var match = String(readerTemplate.fields.time || '').match(/(\d{1,2})[:：](\d{2})/);
    var now = match ? null : new Date();
    var minute = match ? (Number(match[1]) % 24) * 60 + Number(match[2]) : now.getHours() * 60 + now.getMinutes();
    if (minute === clockMinute) return;
    clockMinute = minute;
    var day = minute >= 360 && minute < 1080;
    var phase = minute >= 300 && minute < 450 ? 'dawn' : minute >= 990 && minute < 1140 ? 'dusk' : day ? 'day' : 'night';
    var t = day ? (minute - 360) / 720 : ((minute + 360) % 1440) / 720;
    var root = document.documentElement;
    root.setAttribute('data-mc-phase', phase);
    root.setAttribute('data-mc-celestial', day ? 'sun' : 'moon');
    root.style.setProperty('--mc-celestial-x', (8 + t * 84).toFixed(3) + '%');
    root.style.setProperty('--mc-celestial-y', (72 - Math.sin(t * Math.PI) * 60).toFixed(3) + '%');
    var stop = 0;
    while (stop < skyStops.length - 2 && minute > skyStops[stop + 1][0]) stop++;
    var a = skyStops[stop], b = skyStops[stop + 1], amount = (minute - a[0]) / (b[0] - a[0]);
    root.style.setProperty('--mc-sky-top', colorBetween(a[1], b[1], amount));
    root.style.setProperty('--mc-sky-bottom', colorBetween(a[2], b[2], amount));
    root.style.setProperty('--mc-star-opacity', String(day ? 0 : Math.min(1, Math.max(.2, Math.sin(t * Math.PI)))));
  }
  function updateInventory(page, index) {
    var hotbar = page && page.querySelector('.mc-hotbar');
    if (!hotbar) return;
    var slot = String(((index % 9) + 9) % 9);
    if (hotbar.getAttribute('data-mc-slot') === slot) return;
    hotbar.setAttribute('data-mc-slot', slot);
    hotbar.style.setProperty('--mc-slot', slot);
    // The held item has the same fixed box for all nine textures. Prepare it
    // alongside the hotbar, including pages rendered for the native turn cache.
    var held = page.querySelector('.mc-tool-slot .mc-item');
    var selected = hotbar.querySelectorAll('.mc-item')[Number(slot)];
    if (held && selected) held.className = selected.className;
  }
  function cancelShot() {
    shotAnimations.forEach(function (animation) { animation.onfinish = null; animation.cancel(); });
    shotAnimations = [];
  }
  function cancelMotion() {
    animatedPage = null;
    cancelShot();
    animations.forEach(function (animation) { animation.cancel(); });
    animations = [];
  }
  function animate(node, keyframes, duration, delay) {
    if (!node || typeof node.animate !== 'function') return;
    animations.push(node.animate(keyframes, {duration: duration, delay: delay || 0, iterations: Infinity, easing: 'ease-in-out'}));
  }
  function startShot(page) {
    var arrow = page.querySelector('.mc-arrow');
    if (!arrow || typeof arrow.animate !== 'function' || readerTemplate.viewport.height <= 560) return;
    var duration = 3200 + Math.random() * 1400;
    var distance = (readerTemplate.viewport.width <= 370 ? 24 : 44) + Math.random() * 6 - 3;
    var drop = 5 + Math.random() * 7, arc = 8 + Math.random() * 8;
    var batch = [];
    shotAnimations = batch;
    function shot(node, frames) {
      if (!node) return null;
      var animation = node.animate(frames, {duration: duration, easing: 'linear'});
      batch.push(animation);
      return animation;
    }
    function visibility(start, end, rest) {
      return [{offset: 0, opacity: rest}, {offset: start, opacity: rest},
        {offset: start, opacity: 1 - rest}, {offset: end, opacity: 1 - rest},
        {offset: end, opacity: rest}, {offset: 1, opacity: rest}];
    }
    function path(t) {
      var x = distance * t, y = drop * t - 4 * arc * t * (1 - t);
      var angle = Math.atan2(drop - 4 * arc + 8 * arc * t, distance) * 180 / Math.PI;
      return 'translate(' + x.toFixed(3) + 'px, ' + y.toFixed(3) + 'px) rotate(' + angle.toFixed(3) + 'deg)';
    }
    // Randomize each flight once, then let the compositor interpolate its arc.
    // The clipped range and fixed standing pose never enter the text flow.
    var flight = [{offset: 0, opacity: 0, transform: path(0)},
      {offset: .42, opacity: 0, transform: path(0)}];
    for (var i = 0; i <= 12; i++) {
      flight.push({offset: .42 + .28 * i / 12, opacity: 1, transform: path(i / 12)});
    }
    flight.push({offset: .79, opacity: 1, transform: path(1)},
      {offset: .89, opacity: 0, transform: path(1)}, {offset: 1, opacity: 0, transform: path(1)});
    shot(page.querySelector('.mc-bow-rest'), visibility(.12, .42, 1));
    shot(page.querySelector('.mc-bow-draw-0'), visibility(.12, .22, 0));
    shot(page.querySelector('.mc-bow-draw-1'), visibility(.22, .32, 0));
    shot(page.querySelector('.mc-bow-draw-2'), visibility(.32, .42, 0));
    shot(page.querySelector('.mc-mob-skeleton .mc-mob-arm.mc-limb-back'), [
      {offset: 0, transform: 'rotate(-68deg)'}, {offset: .12, transform: 'rotate(-68deg)'},
      {offset: .32, transform: 'rotate(-56deg)'}, {offset: .42, transform: 'rotate(-56deg)'},
      {offset: .46, transform: 'rotate(-68deg)'}, {offset: 1, transform: 'rotate(-68deg)'}
    ]);
    shot(page.querySelector('.mc-target-flash'), [
      {offset: 0, opacity: 0, transform: 'scale(.8)'}, {offset: .7, opacity: 0, transform: 'scale(.8)'},
      {offset: .72, opacity: .7, transform: 'scale(1)'}, {offset: .87, opacity: 0, transform: 'scale(1.3)'},
      {offset: 1, opacity: 0, transform: 'scale(1.3)'}
    ]);
    var controller = shot(arrow, flight);
    // At most one finite shot exists. A paused/cancelled page cannot enqueue
    // another one, and completed effects are removed before the next shot.
    controller.onfinish = function () {
      if (animatedPage !== page || shotAnimations !== batch) return;
      cancelShot();
      if (readerTemplate.motionState === 'running') startShot(page);
    };
  }
  function startMotion(page) {
    animatedPage = page;
    if (readerTemplate.viewport.height <= 560) return;
    // Web Animations changes only compositor transforms/opacity. No frame loop,
    // geometry reads, DOM writes or timers compete with the reader's touch input.
    // Native page-turn snapshots use the CSS resting pose. Keep the head,
    // torso, shadow and world position fixed in both live and cached frames.
    // Only the creeper's feet step gently; the skeleton stands and draws a bow.
    Array.prototype.forEach.call(page.querySelectorAll('.mc-mob-creeper .mc-mob-leg'), function (limb) {
      var degrees = 9;
      animate(limb, [{transform: 'rotate(0deg)'}, {transform: 'rotate(' + degrees + 'deg)'},
        {transform: 'rotate(0deg)'}, {transform: 'rotate(' + (-degrees) + 'deg)'},
        {transform: 'rotate(0deg)'}], 1800, limb.classList.contains('mc-limb-front') ? -900 : 0);
    });
    Array.prototype.forEach.call(page.querySelectorAll('.mc-motes i, .mc-cave-sparks i'), function (mote, index) {
      animate(mote, [{transform: 'translate(0, 0)', opacity: .15}, {transform: 'translate(4px, -9px)', opacity: .85}, {transform: 'translate(-3px, -18px)', opacity: 0}], 4600 + index * 1100, -index * 900);
    });
    animate(page.querySelector('.mc-cave-lantern'), [{transform: 'rotate(0deg)'}, {transform: 'rotate(3deg)'},
      {transform: 'rotate(0deg)'}, {transform: 'rotate(-3deg)'}, {transform: 'rotate(0deg)'}], 4800);
    startShot(page);
  }
  readerTemplate.on('afterLayout', function (event) {
    cancelMotion(); updateClock();
    event.pages.forEach(function (page, index) {
      updateInventory(page, index);
      var fill = page.querySelector('.mc-xp i');
      if (fill) fill.style.transform = 'scaleX(' + ((index + 1) / event.pages.length) + ')';
    });
  });
  readerTemplate.on('pageChange', function (event) { updateInventory(event.page, event.pageIndex); });
  // Time changes once a minute. Progress/page field updates must not trigger a
  // chapter-wide clock callback or its two-frame geometry barrier on every turn.
  readerTemplate.on('fieldsChange', updateClock, {fields: ['time']});
  readerTemplate.on('motionChange', function (event) {
    if (event.state === 'settled' || event.reducedMotion || animatedPage !== event.page) cancelMotion();
    if (event.state === 'running' && !event.reducedMotion) {
      if (!animatedPage) startMotion(event.page);
      else {
        animations.concat(shotAnimations).forEach(function (animation) { if (animation.playState !== 'finished') animation.play(); });
        if (!shotAnimations.length) startShot(event.page);
      }
    } else if (event.state === 'paused') {
      animations.concat(shotAnimations).forEach(function (animation) { animation.pause(); });
    }
  });
  readerTemplate.on('dispose', cancelMotion);
}());
