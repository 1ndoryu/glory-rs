/* [074A-13] Hook de estado del formulario editor de miembro del equipo.
 * Patrón idéntico a useEditorProyecto pero con campos de Miembro. */

import {useState, useCallback} from 'react';
import {AdminTeamMember, CreateTeamMemberBody, UpdateTeamMemberBody} from '../api/admin-team';

export function useEditorMiembro() {
    const [nombre, setNombre] = useState('');
    const [slug, setSlug] = useState('');
    const [cargo, setCargo] = useState('');
    const [bio, setBio] = useState('');
    const [avatar, setAvatar] = useState('');
    const [linkedin, setLinkedin] = useState('');
    const [twitter, setTwitter] = useState('');
    const [github, setGithub] = useState('');
    const [status, setStatus] = useState('published');
    const [sortOrder, setSortOrder] = useState(0);

    const cargarDesde = useCallback((m: AdminTeamMember) => {
        setNombre(m.name);
        setSlug(m.slug);
        setCargo(m.role);
        setBio(m.bio);
        setAvatar(m.avatar || '');
        setLinkedin(m.linkedin || '');
        setTwitter(m.twitter || '');
        setGithub(m.github || '');
        setStatus(m.status);
        setSortOrder(m.sort_order);
    }, []);

    const resetear = useCallback(() => {
        setNombre('');
        setSlug('');
        setCargo('');
        setBio('');
        setAvatar('');
        setLinkedin('');
        setTwitter('');
        setGithub('');
        setStatus('published');
        setSortOrder(0);
    }, []);

    const buildCreateBody = useCallback((): CreateTeamMemberBody => ({
        name: nombre,
        slug,
        role: cargo || undefined,
        bio: bio || undefined,
        avatar: avatar || undefined,
        linkedin: linkedin || undefined,
        twitter: twitter || undefined,
        github: github || undefined,
        status,
        sort_order: sortOrder,
    }), [nombre, slug, cargo, bio, avatar, linkedin, twitter, github, status, sortOrder]);

    const buildUpdateBody = useCallback((): UpdateTeamMemberBody => ({
        name: nombre || undefined,
        slug: slug || undefined,
        role: cargo || undefined,
        bio: bio || undefined,
        avatar: avatar || undefined,
        linkedin: linkedin || undefined,
        twitter: twitter || undefined,
        github: github || undefined,
        status,
        sort_order: sortOrder,
    }), [nombre, slug, cargo, bio, avatar, linkedin, twitter, github, status, sortOrder]);

    return {
        nombre, setNombre,
        slug, setSlug,
        cargo, setCargo,
        bio, setBio,
        avatar, setAvatar,
        linkedin, setLinkedin,
        twitter, setTwitter,
        github, setGithub,
        status, setStatus,
        sortOrder, setSortOrder,
        cargarDesde, resetear, buildCreateBody, buildUpdateBody,
    };
}
