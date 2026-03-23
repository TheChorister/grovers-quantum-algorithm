import React from 'react';
import classnames from 'classnames';

import * as styles from './Button.css';

export default function Button({ children, className, ...props }) {
    return <div className={classnames(styles.button, className)} { ...props }><a href='#'>{ children }</a></div>
}
