import { invoke } from '@tauri-apps/api/core';
import React, { useEffect, useLayoutEffect } from 'react';

import styles from './PackageList.module.scss';

const PackageList: React.FC<PackageListProps> = (props) => {

    useLayoutEffect(() => {
        (async () => {
            const buckets = await invoke('get_available_buckets');

            console.log(buckets);

        })()
    },[])

    return (
        <div className={styles.container}>
            <div className={styles.buckets}>
                <h3>Buckets</h3>

                <div className={styles.innerContainer}></div>
            </div>

            <div className={styles.apps}>
                <h3>Apps</h3>
                <div className={styles.innerContainer}></div>
            </div>
        </div>
    );
};

interface PackageListProps {
    [key: string]: any;
}

export default PackageList;
