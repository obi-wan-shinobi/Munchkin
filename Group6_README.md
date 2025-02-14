# Munchkin Repository

This repository is a fork of the original **Munchkin** repository from ConSol-Lab. Follow the steps below to clone this fork and set up the upstream remote.

## Cloning the Repository

To clone this repository from your fork, run the following command:

### Using SSH:

```sh
git clone git@github.com:obi-wan-shinobi/Munchkin.git
```

### Using HTTPS:

```sh
git clone https://github.com/obi-wan-shinobi/Munchkin.git
```

Change into the newly created directory:

```sh
cd Munchkin
```

## Adding the Upstream Remote

To keep your fork up to date with the original repository, add it as an upstream remote:

### Using SSH:

```sh
git remote add upstream git@github.com:ConSol-Lab/Munchkin.git
```

### Using HTTPS:

```sh
git remote add upstream https://github.com/ConSol-Lab/Munchkin.git
```

Verify that the remotes are set up correctly:

```sh
git remote -v
```

You should see output similar to:

```
origin    git@github.com:obi-wan-shinobi/Munchkin.git (fetch)
origin    git@github.com:obi-wan-shinobi/Munchkin.git (push)
upstream  git@github.com:ConSol-Lab/Munchkin.git (fetch)
upstream  git@github.com:ConSol-Lab/Munchkin.git (push)
```

## Adding the GitLab Remote

If you also need to push changes to a GitLab repository, add it as another remote:

### Using SSH:

```sh
git remote add gitlab git@gitlab.ewi.tudelft.nl:cs4535/24-25-q3/group-6.git
```

### Using HTTPS:

```sh
git remote add gitlab https://gitlab.ewi.tudelft.nl/cs4535/24-25-q3/group-6.git
```

Verify the remotes again:

```sh
git remote -v
```

## Fetching Updates from Upstream

To sync your fork with the latest changes from the original repository:

1. **Fetch the latest updates from the upstream repository:**

   ```sh
   git fetch upstream
   ```

   This retrieves the latest changes from the upstream repository but does not apply them to your local branch yet.

2. **Switch to your local `main` branch:**

   ```sh
   git checkout main
   ```

3. **Merge the upstream changes into your local `main` branch:**

   ```sh
   git merge upstream/main
   ```

   This integrates the latest updates from the upstream repository into your local branch while preserving your existing commits.

4. **(Optional) If you prefer rebasing instead of merging:**

   ```sh
   git rebase upstream/main
   ```

   Rebasing rewrites your local commits on top of the upstream changes, keeping a cleaner history.

## Pushing Updates to Your Fork and GitLab Repository

If you've merged or rebased, push the changes back to your fork:

```sh
git push origin main
```

To push changes to GitLab as well:

```sh
git push gitlab main
```

Now your fork and GitLab repository are up to date with the original Munchkin repository! 🚀
