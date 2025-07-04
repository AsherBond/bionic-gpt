const COOKIE_NAME = 'flash_aargh';

export const snackBar = () => {
    const snackbar = document.getElementById('snackbar');
    if (!snackbar) {
        return;
    }   
    
    const message = getCookie(COOKIE_NAME);
    if (message) {
        const messageElement = snackbar.querySelector('p');
        if (messageElement) {
            messageElement.textContent = message;
        }

        const hideSnackbar = () => {
            snackbar.classList.remove('translate-y-0', 'opacity-100');
            snackbar.classList.add('translate-y-full', 'opacity-0');
        };

        // Show the snackbar by removing the classes that hide it
        snackbar.classList.remove('translate-y-full', 'opacity-0');
        snackbar.classList.add('translate-y-0', 'opacity-100');

        const dismiss = snackbar.querySelector<HTMLButtonElement>('button.action');
        if (dismiss) {
            dismiss.addEventListener('click', hideSnackbar, { once: true });
        }

        // Delete the cookie after reading its value
        deleteCookie(COOKIE_NAME);

        // Automatically hide the snackbar after 4 seconds
        setTimeout(hideSnackbar, 4000);
    }
};


function getCookie(name: string): string | null {
    const nameLenPlus = name.length + 1;
    return document.cookie
        .split(';')
        .map(c => c.trim())
        .filter(cookie => {
            return cookie.substring(0, nameLenPlus) === `${name}=`;
        })
        .map(cookie => {
            return decodeURIComponent(cookie.substring(nameLenPlus));
        })[0] || null;
}

function deleteCookie(name: string) {
    document.cookie = `${name}=; Max-Age=0; path=/;`;
}