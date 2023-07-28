import React, { useEffect } from 'react';
import styles from './CasperConnector.module.scss';
import If from 'components/If';
import { useRecoilValue } from 'recoil';
import { localizedContentState } from 'store/localizationStore';
import PropTypes from 'prop-types';
import useCasperWallet from 'hooks/useCasperWallet';

const CasperConnector = ({ onWalletChanged }) => {
  const { publicKey, requestConnection, getPublicKey, disconnect, isConnected, isAvailable } = useCasperWallet()

  useEffect(() => {
    onWalletChanged(publicKey)
  }, [publicKey])

  const {
    walletConnector: {
      casperWallet: {
        connectBtn: connectBtnText,
        getKeyBtn: getKeyBtnText,
        disconnectBtn: disconnectBtnText
      }
    }
  } = useRecoilValue(localizedContentState);

  return !isAvailable ? null : (
    <div className={styles.container}>
      <If condition={!isConnected}>
        <button onClick={requestConnection} type="button">{connectBtnText}</button>
      </If>
      <If condition={isConnected && !publicKey}>
        <button onClick={getPublicKey} type="button">{getKeyBtnText}</button>
      </If>
      <If condition={isConnected && Boolean(publicKey)}>
        <button onClick={disconnect} type="button">{disconnectBtnText}</button>
      </If>
    </div>
  )
}

CasperConnector.propTypes = {
  onWalletChanged: PropTypes.func.isRequired
};

export default CasperConnector;
