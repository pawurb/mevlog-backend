document.addEventListener('DOMContentLoaded', () => {
  if (window.location.pathname === '/search') {
    streamOutput('search', ['blocks', 'position', 'from', 'to', 'event', 'not_event', 'method', 'erc20_transfer', 'tx_cost', 'gas_price', 'reverse', 'rpc_url', 'chain_id']);
  }

  if (window.location.pathname === '/trace') {
    streamOutput('trace', ['tx_hash', 'rpc_url']);
  }

  if (window.location.pathname === '/inspect') {
    streamOutput('inspect', ['tx_hash', 'before', 'after', 'reverse', 'rpc_url']);
  }
});

function wsProtocol() {
  return window.location.protocol === 'https:' ? 'wss:' : 'ws:';
}

function appendParamsFromUrl(urlParams, params, paramNames) {
  paramNames.forEach(name => {
    if (urlParams.get(name)) {
      params.append(name, urlParams.get(name));
    }
  });
}

function streamOutput(path, paramsArr) {
  const params = new URLSearchParams();
  const urlParams = new URLSearchParams(window.location.search);

  appendParamsFromUrl(urlParams, params, paramsArr);
  const socket = new WebSocket(`${wsProtocol()}//${window.location.host}/ws/${path}?${params.toString()}`);

  socket.addEventListener('open', (event) => {
    console.log('Connected to WebSocket server');
  });


  const cmdOutput = document.querySelector('.js-cmd-output');
  let isFirstMessage = true;

  socket.addEventListener('message', (event) => {
    console.log('Raw message received:', event.data);
    try {
      // Try to parse as JSON first
      const jsonData = JSON.parse(event.data);
      console.log('Parsed JSON data:', jsonData);

      if (window.updateMevlogViewer) {
        console.log('Updating React with JSON data:', jsonData);
        // Send JSON data to React component
        window.updateMevlogViewer(jsonData);
        // Hide the text output when React takes over
        cmdOutput.style.display = 'none';
      } else {
        console.warn('updateMevlogViewer not available, showing in output');
        if (isFirstMessage) {
          cmdOutput.innerHTML = '';
          isFirstMessage = false;
        }
        cmdOutput.insertAdjacentHTML('beforeend', `<pre>${JSON.stringify(jsonData, null, 2)}</pre>`);
      }
    } catch (e) {
      console.error('Error parsing JSON, raw data:', event.data);
      console.error('JSON parse error:', e);
      // If it's not JSON, display as regular text
      if (isFirstMessage) {
        cmdOutput.innerHTML = '';
        isFirstMessage = false;
      }
      cmdOutput.insertAdjacentHTML('beforeend', `<div>${event.data}</div>`);
    }
  });

  socket.addEventListener('close', (event) => {
    console.log('Disconnected from WebSocket server');
  });

  socket.addEventListener('error', (event) => {
    console.error('WebSocket error:', event);
  });

}

// Make streamOutput globally available
window.streamOutput = streamOutput;