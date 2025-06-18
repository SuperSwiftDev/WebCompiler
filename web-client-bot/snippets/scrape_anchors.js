Array.from(document.querySelectorAll('a'))
    .filter(a => a.href && !a.href.startsWith('javascript:') && a.offsetParent !== null)
    .map(a => ({
        href: a.href,
        text: a.textContent.trim()
    }))