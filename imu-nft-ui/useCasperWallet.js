import { useCallback, useEffect, useRef, useState } from 'react';

const EVENTS = {
  CONNECTED: 'casper-wallet:connected',
  DISCONNECTED: 'casper-wallet:disconnected',
  KEY_CHANGED: 'casper-wallet:activeKeyChanged'
}
const useCasperWallet = () => {
  const providerRef = useRef(window.CasperWalletProvider && window.CasperWalletProvider());
  const intervalRef = useRef(null)
  const [isAvailable, setIsAvailable] = useState(Boolean(window.CasperWalletProvider));

  const checkProvider = useCallback(() => {
    intervalRef.current = setInterval(() => {
      if (window.CasperWalletProvider) {
        providerRef.current = window.CasperWalletProvider()
        setIsAvailable(true)
        clearInterval(intervalRef.current)
      }
    }, 1000)
  }, []);

  useEffect(() => {
    if (!providerRef.current) {
      checkProvider()
    }
    return () => {
      clearInterval(intervalRef.current)
    }
  }, [checkProvider])

  const [isConnected, setIsConnected] = useState(false);
  const [publicKey, setPublicKey] = useState('');

  useEffect(() => {
    if (isAvailable) {
      providerRef.current.isConnected().then(res => {
        setIsConnected(res)
      })
    }
  }, [isAvailable])

  const requestConnection = useCallback(() => {
    providerRef.current.requestConnection().then(res => {
      setIsConnected(res)
    })
  }, []);

  const getPublicKey = useCallback(() => {
    providerRef.current.getActivePublicKey().then(key => {
      setPublicKey(key)
    })
  }, [])

  const disconnect = useCallback(() => {
    providerRef.current.disconnectFromSite().then(res => {
      setIsConnected(!res)
      setPublicKey('')
    })
  }, [])

  useEffect(() => {
    const handleConnected = (event) => {
      try {
        setIsConnected(true)
        const state = JSON.parse(event.detail);
        if (state.activeKey) {
          setPublicKey(state.activeKey);
        } else {
          getPublicKey()
        }
      } catch (err) {
        console.error(err);
      }
    };

    window.addEventListener(EVENTS.CONNECTED, handleConnected);

    return () => {
      window.removeEventListener(EVENTS.CONNECTED, handleConnected);
    };
  }, [setIsConnected, setPublicKey, getPublicKey]);

  useEffect(() => {
    const handleDisconnected = () => {
      try {
        setIsConnected(false)
        setPublicKey('');
      } catch (err) {
        console.error(err);
      }
    };

    window.addEventListener(EVENTS.DISCONNECTED, handleDisconnected);

    return () => {
      window.removeEventListener(EVENTS.DISCONNECTED, handleDisconnected);
    };
  }, [setIsConnected, setPublicKey]);

  useEffect(() => {
    const handleKeyChanged = (event) => {
      try {
        const state = JSON.parse(event.detail);
        if (state.activeKey) {
          setPublicKey(state.activeKey);
        } else {
          getPublicKey()
        }
      } catch (err) {
        console.error(err);
      }
    };

    window.addEventListener(EVENTS.KEY_CHANGED, handleKeyChanged);

    return () => {
      window.removeEventListener(EVENTS.KEY_CHANGED, handleKeyChanged);
    };
  }, [setPublicKey]);

  return {
    isAvailable,
    isConnected,
    requestConnection,
    getPublicKey,
    publicKey,
    disconnect
  }
}

export default useCasperWallet;
