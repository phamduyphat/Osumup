// ==UserScript==
// @name         osum!direct-web
// @version      1.2
// @description  Modify the direct beatmap download button on the osu! site to support the custom osum!direct-web protocol handler without osu!supporter
// @author       oSumAtrIX
// @include      https://osu.ppy.sh/*
// ==/UserScript==

(() => {
	'use strict';
    let button;
    let interval = setInterval(() => {
        if (!location.pathname.includes('/beatmapsets/') || (button = document.querySelector("div.beatmapset-header__buttons > a[href*=support]")) == null) return;
        button.attributes.href.value = "osum-direct-web:?" + location.pathname.split('/')[2]
}, 500);
})();
