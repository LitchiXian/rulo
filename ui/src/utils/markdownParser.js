import markdownIt from 'markdown-it'

export default function markdownParser() {
    return markdownIt({
        html: true,
        linkify: true,
        typographer: true
    })
}