const figlet = require('figlet');

figlet.text('Autumn', {
    font: 'ANSI Shadow',
}, function (err, data) {
    if (err) {
        console.dir(err);
        return;
    }
    console.log(data);
});
