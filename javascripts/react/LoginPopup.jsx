import React from 'react';

const LoginPopup = ({ onClose, onLogin }) => {
  const popupStyle = {
    position: 'fixed',
    top: '0',
    left: '0',
    right: '0',
    bottom: '0',
    backgroundColor: 'rgba(0, 0, 0, 0.8)',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    zIndex: 2000,
    padding: '20px'
  };

  const contentStyle = {
    backgroundColor: '#1a1a1a',
    border: '1px solid #333',
    borderRadius: '8px',
    padding: '32px',
    maxWidth: '500px',
    width: '100%',
    color: '#fff',
    fontFamily: 'monospace',
    textAlign: 'center'
  };

  const titleStyle = {
    color: '#ffd700',
    fontSize: '18px',
    fontWeight: 'bold',
    marginBottom: '16px'
  };

  const messageStyle = {
    fontSize: '14px',
    lineHeight: '1.5',
    marginBottom: '24px',
    color: '#ccc'
  };

  const buttonContainerStyle = {
    display: 'flex',
    gap: '12px',
    justifyContent: 'center'
  };

  const loginButtonStyle = {
    backgroundColor: '#ffd700',
    border: '1px solid #ccc',
    borderRadius: '4px',
    color: '#000',
    cursor: 'pointer',
    fontSize: '14px',
    fontWeight: 'bold',
    padding: '12px 24px',
    transition: 'all 0.2s ease',
    fontFamily: 'monospace'
  };

  const cancelButtonStyle = {
    backgroundColor: '#2a2a2a',
    border: '1px solid #444',
    borderRadius: '4px',
    color: '#fff',
    cursor: 'pointer',
    fontSize: '14px',
    padding: '12px 24px',
    transition: 'all 0.2s ease',
    fontFamily: 'monospace'
  };

  const handleLoginClick = () => {
    if (onLogin) {
      onLogin();
    } else {
      window.location.href = '/auth/github/login';
    }
  };

  const handleBackdropClick = (e) => {
    if (e.target === e.currentTarget) {
      onClose();
    }
  };

  return (
    <div style={popupStyle} onClick={handleBackdropClick}>
      <div style={contentStyle} onClick={(e) => e.stopPropagation()}>
        <div style={titleStyle}>Authentication Required</div>
        <div style={messageStyle}>
          Exploring non-mainnet chains requires authentication to filter out bots and prevent throttling on public RPC endpoints.
          Please sign in with your GitHub account to continue.
        </div>
        <div style={buttonContainerStyle}>
          <button
            onClick={handleLoginClick}
            style={loginButtonStyle}
            onMouseEnter={(e) => e.target.style.backgroundColor = '#e6c200'}
            onMouseLeave={(e) => e.target.style.backgroundColor = '#ffd700'}
          >
            Login with GitHub
          </button>
          <button
            onClick={onClose}
            style={cancelButtonStyle}
            onMouseEnter={(e) => e.target.style.backgroundColor = '#3a3a3a'}
            onMouseLeave={(e) => e.target.style.backgroundColor = '#2a2a2a'}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
};

export default LoginPopup;