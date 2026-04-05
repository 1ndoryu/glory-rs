import React from 'react';
import {spaClick} from '../navegacionSPA';

interface GloryLinkProps extends React.AnchorHTMLAttributes<HTMLAnchorElement> {
    to: string;
}

export const GloryLink: React.FC<GloryLinkProps> = ({to, onClick, ...props}) => {
    const handleClick = (event: React.MouseEvent<HTMLAnchorElement>) => {
        spaClick(event, to);
        onClick?.(event);
    };

    return <a {...props} href={to} onClick={handleClick} />;
};