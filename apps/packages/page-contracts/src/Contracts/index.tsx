// Copyright 2017-2021 @polkadot/app-contracts authors & contributors
// SPDX-License-Identifier: Apache-2.0

import type { AppProps as Props } from '@polkadot/react-components/types';

import React, { useCallback, useEffect, useState } from 'react';
import styled from 'styled-components';

import { Button } from '@polkadot/react-components';
import { useContracts, useToggle } from '@polkadot/react-hooks';

import Codes from '../Codes';
import CodeAdd from '../Codes/Add';
import CodeUpload from '../Codes/Upload';
import ContractAdd from '../Contracts/Add';
import ContractsTable from '../Contracts/ContractsTable';
import store from '../store';
import { useTranslation } from '../translate';
import Deploy from './Deploy';
import Summary from './Summary';

function Contracts ({ className = '' }: Props): React.ReactElement<Props> {
  const { t } = useTranslation();
  const { allContracts } = useContracts();
  const [codeHash, setCodeHash] = useState<string | undefined>();
  const [constructorIndex, setConstructorIndex] = useState(0);
  const [isAddOpen, toggleAdd] = useToggle();
  const [isDeployOpen, toggleDeploy, setIsDeployOpen] = useToggle();
  const [isHashOpen, toggleHash] = useToggle();
  const [isUploadOpen, toggleUpload] = useToggle();
  const [updated, setUpdated] = useState(() => Date.now());
  const [allCodes, setAllCodes] = useState(() => store.getAllCode());

  const _onShowDeploy = useCallback(
    (codeHash: string, constructorIndex: number): void => {
      setCodeHash(codeHash || (allCodes && allCodes[0] ? allCodes[0].json.codeHash : undefined));
      setConstructorIndex(constructorIndex);
      toggleDeploy();
    },
    [allCodes, toggleDeploy]
  );

  const _onCloseDeploy = useCallback(
    () => setIsDeployOpen(false),
    [setIsDeployOpen]
  );

  useEffect(
    (): void => {
      const triggerUpdate = (): void => {
        setUpdated(Date.now());
        setAllCodes(store.getAllCode());
      };

      store.on('new-code', triggerUpdate);
      store.on('removed-code', triggerUpdate);
      store
        .loadAll()
        .then(() => setAllCodes(store.getAllCode()))
        .catch((): void => {
          // noop, handled internally
        });
    },
    []
  );

  return (
    <div className={className}>
    </div>
  );
}

export default React.memo(styled(Contracts)`
  .ui--Table td > article {
    background: transparent;
    border: none;
    margin: 0;
    padding: 0;
  }
`);
