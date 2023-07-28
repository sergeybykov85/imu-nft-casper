import React from 'react';
import PropTypes from 'prop-types';
import { SYSTEM_NAMES, SYSTEMS } from 'constants/blockchains';
import { CustomConnectBtn } from 'components/RainbowKit';
import CasperConnector from 'components/CasperConnector';

const WalletConnector = ({ system, onWalletChanged }) => {
  switch (system) {
    case SYSTEMS.ethereum:
    case SYSTEMS.polygon:
    case SYSTEMS.bsc:
      return (
        <CustomConnectBtn onConnected={onWalletChanged} system={system} />
      )
    case SYSTEMS.casper:
      return (
        <CasperConnector { ...{ onWalletChanged } } />
      )
    default:
      return null
  }
};

WalletConnector.propTypes = {
  system: PropTypes.oneOf(SYSTEM_NAMES).isRequired,
  onWalletChanged: PropTypes.func.isRequired
};

export default WalletConnector;
