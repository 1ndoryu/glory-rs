/*
 * Datos de blog centralizados.
 * [044A-4] Post real sobre Kamples. Placeholders eliminados.
 */
import {PostBlog} from '../types/contenido';

const POSTS_FALLBACK: PostBlog[] = [
    {
        id: 1,
        titulo: 'Kamples: Reimagining How Musicians Discover and Share Samples',
        resumen: 'An open-source platform that combines a sample library, recommendation engine, lightweight DAW, and social features — think WhoSampled meets Pinterest for music production.',
        contenido: `
            <p>Music production today relies heavily on samples — but discovering, organizing, and sharing them remains fragmented across dozens of tools and communities. <strong>Kamples</strong> is our answer to that problem: a single platform where producers can explore, preview, and share samples with the depth of WhoSampled and the visual curation of Pinterest.</p>

            <h2>The Problem</h2>
            <p>Producers juggle between sample marketplaces, DAWs, and social media to find what they need. There's no single place that lets you <em>discover</em> samples algorithmically, <em>preview</em> them in context, and <em>share</em> collections with your community — all in one flow.</p>

            <h2>What Kamples Does</h2>
            <p>At its core, Kamples is a sample library with three layers built on top:</p>
            <ul>
                <li><strong>Recommendation Engine</strong> — An algorithm that surfaces samples based on genre, mood, BPM, key, and your usage history. The more you interact, the better it gets.</li>
                <li><strong>Integrated DAW</strong> — A lightweight, browser-based workstation for previewing samples in context. Layer, mix, and test before downloading. No need to leave the platform.</li>
                <li><strong>Social Layer</strong> — Organize samples into collections and boards (like Pinterest). Follow other producers, see what they're sampling, and explore "more ideas like this" chains that surface unexpected connections.</li>
            </ul>

            <h2>Open Source</h2>
            <p>Kamples is fully open source. We believe tools for creative expression should be transparent and community-driven. Contributions are welcome — from algorithm improvements to UI components to new audio processing features.</p>

            <h2>Try It</h2>
            <p>Kamples is live at <strong>kamples.com</strong>. Sign up, upload your first sample, and start building your library. The platform is free for individual producers — we're exploring sustainability models for teams and commercial use.</p>
        `,
        fecha: 'Apr 4, 2026',
        categoria: 'Product',
        link: '/blog/kamples-reimagining-sample-discovery',
        imagen: '/assets/Proyectos portadas/Kamples portada.jpg'
    }
];

export const POSTS_BLOG: PostBlog[] = POSTS_FALLBACK;
