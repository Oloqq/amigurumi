export function download(data: any, filename: string, type: string) {
  var file = new Blob([data], { type: type });
  if ((window.navigator as any).msSaveOrOpenBlob) // IE10+
    (window.navigator as any).msSaveOrOpenBlob(file, filename);
  else { // Others
    var a = document.createElement("a"),
      url = URL.createObjectURL(file);
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    setTimeout(function () {
      document.body.removeChild(a);
      window.URL.revokeObjectURL(url);
    }, 0);
  }
}