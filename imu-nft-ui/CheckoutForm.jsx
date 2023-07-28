import React, { useMemo, useState } from 'react';
import PropTypes from 'prop-types';
import Input from 'components/Input';
import { useFormik } from 'formik';
import * as Yup from 'yup';
import { CardElement, useElements, useStripe } from '@stripe/react-stripe-js';
import PaymentService from 'services/paymentService';
import Button from 'components/Button';
import LoadSpinner from 'components/LoadSpinner';
import If from 'components/If';
import { useParams } from 'react-router-dom';
import { useRecoilValue } from 'recoil';
import SuccessMessage from 'components/SuccessMessage';
import { imuDataState } from 'store/imuStore';
import { locationUrlState } from 'store/widgetStore';
import styles from './CheckoutForm.module.scss';
import clsx from 'clsx';
import ToggleSwitch from 'components/ToggleSwitch';
import { localizedContentState } from 'store/localizationStore';
import { SUPPORTED_SYSTEMS, SYSTEMS } from 'constants/blockchains';
import BlockchainIcon from 'components/BlockchainIcon';
import Collapse from 'components/Collapse';
import StripeLogo from 'components/StripeLogo/StripeLogo';
import WalletConnector from 'containers/WalletConnector';

const cardElementOptions = {
  style: {
    base: {
      fontSize: '14px',
      lineHeight: '28px',
      '::placeholder': {
        color: '#B9BFC5',
      },
    },
  },
  hidePostalCode: true
};
const CheckoutForm = ({
  contentId,
  system = '',
  onSuccess = () => {},
  selfPickUpAllowed,
  freeNft,
}) => {
  /* External stores */
  const { imuCode } = useParams();
  const { partnerId } = useRecoilValue(imuDataState);
  const locationUrl = useRecoilValue(locationUrlState);

  /* Local states */
  const [processing, setProcessing] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState(false);
  const [cardInputComplete, setCardInputComplete] = useState(false);
  const [selfPickUp, setSelfPickUp] = useState(false);
  const [connected, setConnected] = useState(false);

  const walletInputAvailable = SUPPORTED_SYSTEMS.includes(system);

  const stripe = !freeNft && useStripe();
  const stripeElements = !freeNft && useElements();

  const service = new PaymentService();

  const createPaymentMethod = async () => {
    const { paymentMethod, error } = await stripe.createPaymentMethod({
      type: 'card',
      card: stripeElements.getElement(CardElement),
      billing_details: {
        name: formik.values.userName,
        email: formik.values.userEmail,
      },
    });
    if (error) {
      return Promise.reject(error);
    }
    return Promise.resolve(paymentMethod);
  };

  const getFree = async () => {
    try {
      const { userName, userEmail, userWallet } = formik.values;
      const fetchData = {
        userName,
        userEmail,
        entityId: contentId,
        sourceUrl: locationUrl,
        sourceType: 'Imu',
      };
      if (walletInputAvailable) { fetchData.userWallet = userWallet }
      const { error } = await service.getFree(fetchData);
      if (error) {
        return Promise.reject({ message: error });
      }
      return Promise.resolve();

    } catch (e) {
      return Promise.reject({
        message: e.errorDetails && e.errorDetails.join('\r\n'),
      });
    }
  };

  const paymentCharge = async ({
    paymentMethodId,
    paymentIntent,
    paymentDecline,
    selfPickUp,
  } = {}) => {
    try {
      const { userName, userEmail, userWallet } = formik.values;
      const fetchData = {
        userName,
        userEmail,
        entityId: contentId,
        sourceUrl: locationUrl,
        sourceCode: imuCode,
        sourceType: 'Imu',
        sourcePartner: partnerId,
      };

      if (paymentMethodId)                     { fetchData.paymentMethod  = paymentMethodId }
      if (paymentIntent)                       { fetchData.paymentIntent  = paymentIntent   }
      if (paymentDecline)                      { fetchData.paymentDecline = true            }
      if (selfPickUp)                          { fetchData.selfPickUp     = true            }
      if (walletInputAvailable && !selfPickUp) { fetchData.userWallet     = userWallet      }

      const { error, requiresAction, status, clientSecret } =
        await service.paymentCharge(fetchData);
      if (error) {
        return Promise.reject({ message: error });
      }
      return Promise.resolve({ requiresAction, status, clientSecret });
    } catch (e) {
      return Promise.reject({
        message: e.errorDetails && e.errorDetails.join('\r\n'),
      });
    }
  };

  const handle3DSecure = async (clientSecret) => {
    const { paymentIntent, error } = await stripe.handleCardAction(
      clientSecret
    );
    if (error) {
      return await paymentCharge({
        paymentIntent: error.payment_intent.id,
        paymentDecline: true,
      });
    }
    return await paymentCharge({
      paymentIntent: paymentIntent.id,
    });
  };

  const onSubmit = async () => {
    if (processing || (!freeNft && !cardInputComplete)) {
      return;
    }
    try {
      setProcessing(true);
      if (freeNft) {
        await getFree()
        return setSuccess(true);
      }
      const paymentMethod = await createPaymentMethod();
      let paymentChargeData = { paymentMethodId: paymentMethod.id };
      if (selfPickUp) {
        paymentChargeData.selfPickUp = true;
      }
      const { requiresAction, status, clientSecret } = await paymentCharge(
        paymentChargeData
      );
      if (status === 'succeeded' && !requiresAction) {
        return setSuccess(true);
      }
      if (requiresAction && clientSecret) {
        await handle3DSecure(clientSecret);
      }
      setSuccess(true);
    } catch (e) {
      setProcessing(false);
      setError(e.message || 'Error');
    }
  };

  const initiateFormik = () => {
    const formData = {
      initialValues: {
        userName: '',
        userEmail: '',
      },
      validateOnBlur: false,
      validateOnChange: false,
      onSubmit,
    };
    const validationSchema = {
      userName: Yup.string().required('Required'),
      userEmail: Yup.string()
        .email('Invalid email address')
        .required('Required'),
    };
    if (walletInputAvailable) {
      formData.initialValues.userWallet = '';
      validationSchema.userWallet = selfPickUp ? Yup.string() : Yup.string().required('Required');
    }
    return { ...formData, validationSchema: Yup.object(validationSchema) };
  };
  const formik = useFormik(initiateFormik());

  const resetFieldError = fieldName => {
    setError('');
    const errors = { ...formik.errors };
    delete errors[fieldName];
    formik.setErrors(errors);
  }
  const onInputChange = (e) => {
    resetFieldError(e.target.name)
    formik.handleChange(e);
  };
  const onWalletChanged = address => {
    formik.setFieldValue('userWallet', address ?? '');
    resetFieldError('userWallet')
    setConnected(Boolean(address))
  }
  const onCardInputChange = ({ complete }) => {
    if (!processing) {
      setError('');
      setCardInputComplete(complete);
    }
  };
  const onChangeSelfPickUp = ({ target: { checked } }) => {
    setSelfPickUp(checked);
  };

  const {
    checkoutForm: {
      titlePaid,
      titleFree,
      inputName,
      inputEmail,
      inputWallet,
      inputWalletCasper,
      walletToggle,
      buyBtn,
      claimBtn
    }
  } = useRecoilValue(localizedContentState);

  const walletInputPlaceHolder = useMemo(
    () => system === SYSTEMS.casper ? inputWalletCasper : inputWallet.replace('{system}', system),
    [system]);

  if (success) {
    return <SuccessMessage onClick={onSuccess} selfPickUp={selfPickUp} freeNft={freeNft} />;
  }
  return (
    <div className={styles.container}>
      <h3 className={styles.title}>{freeNft ? titleFree : titlePaid}</h3>
      <If condition={Boolean(error)}>
        <p className={styles.error}>{error}</p>
      </If>
      <form onSubmit={formik.handleSubmit}>
        <Input
          className={styles.input}
          name="userName"
          value={formik.values.userName}
          placeholder={inputName}
          onChange={onInputChange}
          error={Boolean(formik.errors.userName)}
          disabled={processing}
        />
        <Input
          className={styles.input}
          name="userEmail"
          type="email"
          value={formik.values.userEmail}
          placeholder={inputEmail}
          onChange={onInputChange}
          error={Boolean(formik.errors.userEmail)}
          disabled={processing}
        />
        <If condition={selfPickUpAllowed}>
          <ToggleSwitch
            className={styles.toggle}
            checked={selfPickUp}
            onChange={onChangeSelfPickUp}
            label={walletToggle}
          />
        </If>
        <Collapse open={walletInputAvailable && !selfPickUp}>
          <WalletConnector { ...{ system, onWalletChanged } }/>
          <div className={styles['wallet-input-wrapper']}>
            <Input
              className={styles.input}
              name="userWallet"
              value={formik.values.userWallet}
              placeholder={walletInputPlaceHolder}
              onChange={onInputChange}
              disabled={connected}
              error={Boolean(formik.errors.userWallet)}
            />
            <BlockchainIcon className={styles['system-logo']} systemName={system} />
          </div>
        </Collapse>

        <If condition={!freeNft}>
          <div className={clsx(styles.input, styles['card-input-wrapper'])}>
            <CardElement
              options={cardElementOptions}
              onChange={onCardInputChange}
            />
          </div>
        </If>
        <Button text={!freeNft ? buyBtn : claimBtn} type="submit" />
        <If condition={!freeNft}>
          <StripeLogo className={styles['stripe-logo']}/>
        </If>
      </form>
      <If condition={processing}>
        <LoadSpinner className={styles.processing} />
      </If>
    </div>
  );
};

CheckoutForm.propTypes = {
  contentId: PropTypes.number,
  system: PropTypes.string,
  onSuccess: PropTypes.func,
  selfPickUpAllowed: PropTypes.bool,
  freeNft: PropTypes.bool,
};

export default CheckoutForm;
