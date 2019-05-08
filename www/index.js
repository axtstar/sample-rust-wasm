//import * as wasm from "hello-wasm-pack";
import * as sample from "sample-wasm";
import * as bg from "sample-wasm/sample_wasm_bg";

function fib(n) {
    return n < 2 ? n : fib(n - 2) + fib(n - 1);
}

var app = new Vue({
    el: '#app',
    data: {
      number:10,
      message: '',
      canvas: null,
      duration: ''
    },
    methods: {
      greet: function() {
        sample.greet(this.text);
      },
      fib_wasm: function() {
        var start = Date.now();
        this.message = 'fib(' + this.number + ') = ' + sample.fib(this.number) + '!'
        var end = Date.now();
        this.duration = (end - start) + 'ms';
      },
      fib_js: function() {
        var start = Date.now();
        this.message = 'fib(' + this.number + ') = ' + fib(this.number) + '!'
        var end = Date.now();
        this.duration = (end - start) + 'ms';
      },
      to_transparent: function() {
        var start = Date.now();

        const canvas = document.querySelector('canvas');

        const screen = new sample.Screen(canvas.width, canvas.height);
        const ctx = canvas.getContext('2d');

        var imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        sample.to_transparent(screen, imageData.data);
        const image = new ImageData(imageData.data, screen.width, screen.height);
            
        ctx.putImageData(image, 0, 0);

        var end = Date.now();
        this.duration = (end - start) + 'ms';

      },
      uploadFile: function(e){
        var canvas = document.querySelector('canvas');
        var ctx = canvas.getContext("2d");
    
        let files = e.target.files;
        var file = files[0];
    
        var image = new Image();
        var reader = new FileReader();
    
        reader.onload = function(evt) {    
          image.onload = function() {
            ctx.drawImage(image, 0, 0);
          }
          image.src = evt.target.result;
        }
        reader.readAsDataURL(file);
      },
    }
})
