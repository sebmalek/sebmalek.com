window.setTimeout(() => {
    let a = window.atob('bW9jLmtlbGFtYmVzQHRjYXRub2M6b3RsaWFt');
    document.getElementById('m').href = a.split('').reverse().join('');
}, 15);
