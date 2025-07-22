<footer class="footer">
  <div class="footer-contet">
    <p>&copy; josef244 on Codeberg</p>
  </div>
  <a target="_blank" rel="noopener noreferrer" href="https://codeberg.org/josef244/OpenHousing">Codeberg</a>
</footer>


<style>
    .footer {
        background-color: #333;
        color: white;
        padding: 2rem 0;
        margin-top: auto; /* Pushes footer to bottom if using flex layout */
    }

    .footer-content {
        max-width: 1200px;
        margin: 0 auto;
        padding: 0 1rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
    }

    .footer-links {
        display: flex;
        gap: 1.5rem;
    }

    .footer-links a {
        color: white;
        text-decoration: none;
    }

    .footer-links a:hover {
        text-decoration: underline;
    }

    @media (max-width: 768px) {
        .footer-content {
            flex-direction: column;
            text-align: center;
        }

        .footer-links {
            flex-direction: column;
            gap: 0.5rem;
        }
    }
</style>